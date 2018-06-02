use tokio::net::TcpListener;
use futures::Future;
use futures::Async;
use futures::Poll;
use tokio::net::TcpStream;
use tokio::io;


/// returned by `TcpListenerExt::first_incomming`
pub struct FirstIncoming {
    inner: TcpListener,
}

impl FirstIncoming {
    pub fn new(listener: TcpListener) -> FirstIncoming { FirstIncoming { inner: listener } }
}

impl Future for FirstIncoming {
    type Item = TcpStream;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (socket, _) = try_ready!(self.inner.poll_accept());
        Ok(Async::Ready(socket))
    }
}


pub trait TcpListenerExt {
    /// accept the first incomming tcp client
    fn first_incoming(self) -> FirstIncoming;
}

impl TcpListenerExt for TcpListener {
    fn first_incoming(self) -> FirstIncoming { FirstIncoming::new(self) }
}