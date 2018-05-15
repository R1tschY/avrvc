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
use tokio::prelude::stream::{SplitSink, SplitStream};
use futures::sync::mpsc::SendError;
use core::AvrVmInfo;


type Tx = mpsc::UnboundedSender<GdbServerPkt>;
type Rx = mpsc::UnboundedReceiver<GdbServerPkt>;


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

    pub fn new(info: &AvrVmInfo) -> Self {
        let (client_tx, client_rx) = mpsc::unbounded();
        let (server_tx, server_rx) = mpsc::unbounded();

        RemoteServer {
            client_tx,
            server_rx,
            client_rx: Arc::new(Mutex::new(client_rx)),
            server_tx: server_tx,
            debugger: GdbDebugger::new(info),
            commands: GdbCommands::new()
        }
    }

    fn execute_command(&mut self, command: &GdbServerPkt) {
        if let &GdbServerPkt::Packet(_) = command {
            self.client_tx.start_send(GdbServerPkt::Ack { okay: true });
        }

        if let Some(reply) = self.commands.handle(command, &mut self.debugger) {
            self.client_tx.start_send(reply);
        }
    }
}


pub fn serve(info: &AvrVmInfo, addr: &SocketAddr, runtime: &mut Runtime) {
    let socket = TcpListener::bind(addr).unwrap();
    let server = RemoteServer::new(info);
    let client_rx = server.client_rx.clone();
    let server_tx = server.server_tx.clone();

    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let framed = socket.framed(GdbServerCodec::new());
            let (writer, reader) = framed.split();

            tokio::spawn(
                server_tx
                    .clone()
                    .sink_map_err(|e| ())
                    .send_all(reader.map_err(|e| ()))
                    .then(|_| Ok(())));

            let client_rx = client_rx.clone();
            tokio::spawn(
                writer
                    .sink_map_err(|e| ())
                    .send_all(LockedStream(client_rx).map_err(|e| ()))
                    .then(|_| Ok(())))
        });

    let debug = server.map_err(|err| {
        println!("ERROR: {:?}", err);
    });

    // Start the runtime and spin up the server
    runtime.spawn(done);
    runtime.spawn(debug);
}