use std::sync::{Arc, Mutex};

use futures::sync::mpsc;
use tokio::net::TcpListener;
use tokio::io;
use tokio_io::AsyncRead;
use tokio::runtime::Runtime;
use tokio;

use gdb::GdbServerPkt;
use gdb::codec::GdbServerCodec;
use gdb::commands::GdbCommands;
use gdb::debugger::GdbDebugger;
use futures::{Async, Poll, Future, Stream, Sink};
use std::net::SocketAddr;
use core::AvrVm;
use gdb::TcpListenerExt;


pub type Tx = mpsc::UnboundedSender<GdbServerPkt>;
pub type Rx = mpsc::UnboundedReceiver<GdbServerPkt>;


struct LockedStream<S: ?Sized>(Arc<Mutex<S>>);

impl<S: Stream> Stream for LockedStream<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        self.0.lock().unwrap().poll()
    }
}

struct RemoteServer {
    client_tx: Tx,
    server_rx: Rx,
    pub server_tx: Tx,
    pub client_rx: Arc<Mutex<Rx>>,
    debugger: GdbDebugger,
    commands: GdbCommands
}

impl Future for RemoteServer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use gdb::debugger::DebuggerState::*;

        loop {
            match self.debugger.get_state() {
                Running =>
                    // performance opt.: only check state after it could change
                    loop {
                        if !self.debugger.step() {
                            self.client_tx.start_send(
                                GdbServerPkt::Packet(self.debugger.signal_reply())).unwrap();
                            break;
                        }

                        // poll server rx
                        match self.server_rx.poll().unwrap() {
                            Async::Ready(Some(msg)) => {
                                // new message
                                self.execute_command(&msg);
                                if self.debugger.get_state() != Running {
                                    break;
                                }
                            },
                            Async::Ready(None) => {
                                // server RX closed
                                return Ok(Async::Ready(()))
                            },
                            Async::NotReady => {
                                // no message
                            }
                        }
                    },

                Stopped =>
                    match self.server_rx.poll().unwrap() {
                        Async::Ready(Some(msg)) => self.execute_command(&msg),
                        Async::Ready(None) => return Ok(Async::Ready(())),
                        Async::NotReady => return Ok(Async::NotReady),
                    },

                Detached | Killed => {
                    // TODO: Detached -> let it run
                    self.server_rx.close();
                    self.client_rx.lock().unwrap().close();
                    return Ok(Async::Ready(()))
                },
            }
        }
    }
}

impl RemoteServer {

    pub fn new(vm: AvrVm) -> Self {
        let (client_tx, client_rx) = mpsc::unbounded();
        let (server_tx, server_rx) = mpsc::unbounded();
        let client_tx_copy = client_tx.clone();

        RemoteServer {
            client_tx,
            server_rx,
            client_rx: Arc::new(Mutex::new(client_rx)),
            server_tx,
            debugger: GdbDebugger::new(vm),
            commands: GdbCommands::new(client_tx_copy)
        }
    }

    fn execute_command(&mut self, command: &GdbServerPkt) {
        if let &GdbServerPkt::Packet(_) = command {
            self.client_tx.start_send(GdbServerPkt::Ack { okay: true }).unwrap();
        }

        if let Some(reply) = self.commands.handle(command, &mut self.debugger) {
            self.client_tx.start_send(reply).unwrap();
        }
    }
}


pub fn serve(vm: AvrVm, addr: &SocketAddr, runtime: &mut Runtime) {
    let socket = TcpListener::bind(addr).unwrap();
    let server = RemoteServer::new(vm);
    let client_rx = server.client_rx.clone();
    let server_tx = server.server_tx.clone();

    let client = socket
        .first_incoming()  // TODO: accept only one client at a time
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .and_then(move |socket| {
            debug!("accepted client");
            let framed = socket.framed(GdbServerCodec::new());
            let (writer, reader) = framed.split();

            tokio::spawn(
                server_tx
                    .clone()
                    .sink_map_err(|_| ()) // TODO: error handling
                    .send_all(reader.map_err(|_| ())) // TODO: error handling
                    .then(|_| Ok(()))); // TODO: error handling

            let client_rx = client_rx.clone();
            tokio::spawn(
                writer
                    .sink_map_err(|_| ()) // TODO: error handling
                    .send_all(LockedStream(client_rx).map_err(|_| ())) // TODO: error handling
                    .then(|_| Ok(()))) // TODO: error handling
        });

    let debugger = server.map_err(|err| {
        error!("server error: {}", err);
    });

    // Start the runtime and spin up the server
    runtime.spawn(client);
    runtime.spawn(debugger);
}