
#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate avrvc;
extern crate tokio;
extern crate tokio_io;
extern crate futures;
extern crate bytes;

use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::io;
use futures::sync::mpsc;
use docopt::Docopt;
use bytes::{ Bytes};
use std::sync::{Arc, Mutex};

use avrvc::gdb::codec::GdbServerCodec;
use avrvc::gdb::package::GdbServerPkt;
use tokio_io::codec::Framed;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;


const USAGE: &'static str = "
Naval Fate.

Usage:
  avrvc-gdbserver <name> [--arch=<arch>] [--port=<port>]

Options:
  -h --help      Show this screen.
  --version      Show version.
  --arch=<arch>  Architecture
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_name: Vec<String>,
}

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
                    self.gdb_stream.start_send(GdbServerPkt::Ack { okay: true })?;
                    self.client_interface.lock().unwrap().server_tx.unbounded_send(bytes.freeze()).unwrap();
                }
                _ => { }
            }
        }

        Ok(Async::NotReady)
    }
}


struct Debugger {
    client_tx: Tx,
    server_rx: Rx,
    client_interface: Arc<Mutex<Shared>>,
    state: DebuggerState
}

#[derive(Copy, Clone, PartialEq)]
enum DebuggerState {
    Running,
    Stopped
}

impl Future for Debugger {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.state {
                DebuggerState::Running => {

                    loop {
                        // TODO: execute instr

                        // poll server rx
                        match self.server_rx.poll().unwrap() {
                            Async::Ready(Some(msg)) => {
                                // TODO: handle message
                                println!("RUNNING: {:#?}", msg);

                                if self.state != DebuggerState::Running {
                                    break
                                }
                            },
                            Async::Ready(None) => return Ok(Async::Ready(())),
                            Async::NotReady => {
                                // do nothing
                            }
                        }
                    }

                },

                DebuggerState::Stopped => {
                    match self.server_rx.poll().unwrap() {
                        Async::Ready(Some(msg)) => {
                            // TODO: handle message
                            println!("STOPPED: {:#?}", msg);
                        },
                        Async::Ready(None) => {
                            println!("STOPPED: Async::Ready(None)");
                            return Ok(Async::Ready(()))
                        },
                        Async::NotReady => {
                            println!("STOPPED: Async::NotReady");
                            return Ok(Async::NotReady)
                        }
                    }
                }
            }
        }
    }
}

impl Debugger {

    pub fn new() -> Self {
        let (client_tx, client_rx) = mpsc::unbounded();
        let (server_tx, server_rx) = mpsc::unbounded();

        let client_interface = Arc::new(Mutex::new(
            Shared::new(server_tx, client_rx)));

        Debugger {
            client_tx,
            server_rx,
            client_interface,
            state: DebuggerState::Stopped
        }
    }

    pub fn get_client_interface(&self) -> Arc<Mutex<Shared>> {
        self.client_interface.clone()
    }
}

fn main() {
    let _args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

//    let flash = read_executable_file(&args.arg_name[0]);
//    let info = AvrVmInfo {
//        pc_bytes: 3, xmega: true, flash_bytes: flash.len(), memory_bytes: 0x70D0 };
//    let mut state = AvrState { core: AvrVm::new(&info) };
//    let decoder = AvrDecoder::new();
//
//    loop {
//        let instr = decoder.decode(&flash, state.core.pc);
//        instr.execute(&mut state.core);
//    }

    let addr = "127.0.0.1:9000".parse().unwrap();
    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {} ...", addr);
    let debugger = Debugger::new();
    let client_interface = debugger.get_client_interface();

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

    let debug = debugger.map_err(|err| {
        println!("ERROR: {:?}", err);
    });

    // Start the runtime and spin up the server
    let mut runtime = Runtime::new().unwrap();
    runtime.spawn(done);
    runtime.spawn(debug);
    runtime.shutdown_on_idle().wait().unwrap();
}
