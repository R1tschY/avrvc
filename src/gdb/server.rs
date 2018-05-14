use std::sync::{Arc, Mutex};

use bytes::{Bytes};
use futures::sync::mpsc;
use tokio::net::TcpListener;
use tokio::io;
use tokio_io::codec::Framed;
use tokio_io::AsyncRead;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio;

use gdb::GdbServerPkt;
use gdb::codec::GdbServerCodec;
use gdb::commands::GdbCommands;
use gdb::debugger::GdbDebugger;
use futures::{Async, Poll, Future, Stream, Sink};
use std::net::SocketAddr;
use gdb::debugger::DebuggerState;


type Tx = mpsc::UnboundedSender<Bytes>;
type Rx = mpsc::UnboundedReceiver<Bytes>;

struct Shared {
    pub server_tx: Tx,
    pub client_rx: Rx,
}

impl Shared {
    pub fn new(server_tx: Tx, client_rx: Rx) -> Self {
        Shared { server_tx, client_rx }
    }
}

struct RemoteClient {
    client_interface: Arc<Mutex<Shared>>,
    gdb_stream: Framed<TcpStream, GdbServerCodec>,
}

impl RemoteClient {
    pub fn new(
        client_interface: Arc<Mutex<Shared>>,
        gdb_stream: Framed<TcpStream, GdbServerCodec>
    ) -> Self {
        RemoteClient { client_interface, gdb_stream }
    }
}

impl Future for RemoteClient {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        while let Async::Ready(pkt) = self.gdb_stream.poll()? {
            match pkt {
                Some(GdbServerPkt::Packet(bytes)) => {
                    println!("IN: {}", String::from_utf8_lossy(&bytes));
                    self.client_interface.lock().unwrap().server_tx.unbounded_send(
                        bytes).unwrap();
                    self.gdb_stream.start_send(GdbServerPkt::Ack { okay: true })?;
                }
                _ => { } // TODO: what to do when channel closes?
            }
        }

        while let Async::Ready(pkt) = self.client_interface.lock().unwrap().client_rx.poll().unwrap() {
            match pkt {
                Some(bytes) => {
                    println!("OUT: {}", String::from_utf8_lossy(&bytes));
                    self.gdb_stream.start_send(GdbServerPkt::Packet(bytes))?;
                }
                None => { } // TODO: what to do when channel closes?
            }
        }

        Ok(Async::NotReady)
    }
}

struct RemoteServer {
    client_tx: Tx,
    server_rx: Rx,
    client_interface: Arc<Mutex<Shared>>,
    debugger: GdbDebugger,
    commands: GdbCommands
}

impl Future for RemoteServer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.debugger.get_state() {
                DebuggerState::Running =>
                    // performance opt.: only check state after it could change
                    loop {
                        self.debugger.istep();

                        // poll server rx
                        match self.server_rx.poll().unwrap() {
                            Async::Ready(Some(msg)) => {
                                self.execute_command(&msg);
                                if self.debugger.get_state() != DebuggerState::Running {
                                    break
                                }
                            },
                            Async::Ready(None) => return Ok(Async::Ready(())),
                            Async::NotReady => {
                                // do nothing
                            }
                        }
                    },

                DebuggerState::Stopped =>
                    match self.server_rx.poll().unwrap() {
                        Async::Ready(Some(msg)) => {
                            self.execute_command(&msg);
                        },
                        Async::Ready(None) => {
                            return Ok(Async::Ready(()))
                        },
                        Async::NotReady => {
                            return Ok(Async::NotReady)
                        }
                    }
            }
        }
    }
}

impl RemoteServer {

    pub fn new() -> Self {
        let (client_tx, client_rx) = mpsc::unbounded();
        let (server_tx, server_rx) = mpsc::unbounded();

        let client_interface = Arc::new(Mutex::new(
            Shared::new(server_tx, client_rx)));

        RemoteServer {
            client_tx,
            server_rx,
            client_interface,
            debugger: GdbDebugger::new(),
            commands: GdbCommands::new()
        }
    }

    pub fn get_client_interface(&self) -> Arc<Mutex<Shared>> {
        self.client_interface.clone()
    }

    fn execute_command(&mut self, command: &Bytes) {
        if let Some(reply) = self.commands.handle(command, &mut self.debugger) {
            self.client_tx.start_send(reply);
        }
    }
}

pub fn serve(addr: &SocketAddr, runtime: &mut Runtime) {
    let socket = TcpListener::bind(addr).unwrap();
    let server = RemoteServer::new();
    let client_interface = server.get_client_interface();

    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // Once we're inside this closure this represents an accepted client
            // from our server. The `socket` is the client connection (similar to
            // how the standard library operates).
            //
            // We're parsing each socket with the `BytesCodec` included in `tokio_io`,
            // and then we `split` each codec into the reader/writer halves.
            //
            // See https://docs.rs/tokio-io/0.1/src/tokio_io/codec/bytes_codec.rs.html
            let framed = socket.framed(GdbServerCodec::new());
            //let (writer, reader) = framed.split();

            let client = RemoteClient::new(client_interface.clone(), framed);

            let processor = client
                // After our copy operation is complete we just print out some helpful
                // information.
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socket closed with error: {:?}", err);
                    // We have to return the error to catch it in the next ``.then` call
                    Err(err)
                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    Ok(())
                });

            tokio::spawn(processor)
        });

    let debug = server.map_err(|err| {
        println!("ERROR: {:?}", err);
    });

    // Start the runtime and spin up the server
    runtime.spawn(done);
    runtime.spawn(debug);
}