use crate::error::Error;
use crate::traits::Connection;
use async_trait::async_trait;
use std::net::SocketAddr;
use std::result::Result;
use std::sync::Arc;
use tokio::io;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct TcpConnection {
    remote_address: std::net::SocketAddr,
    _blocking: bool,
    stream: Option<tokio::net::TcpStream>,
}

impl TcpConnection {
    pub fn create(remote_address: SocketAddr) -> Arc<Mutex<dyn Connection + Send>> {
        Arc::new(Mutex::new(TcpConnection {
            remote_address,
            _blocking: true,
            stream: None,
        }))
    }
    pub async fn new_from_stream(stream: TcpStream) -> Result<Arc<Mutex<Self>>, Error> {
        match stream.peer_addr() {
            Ok(peer) => Ok(Arc::new(Mutex::new(TcpConnection {
                remote_address: peer,
                _blocking: true,
                stream: Some(stream),
            }))),
            Err(_) => Err(Error::PeerNotFound.into()),
        }
    }
}

#[async_trait]
impl Connection for TcpConnection {
    async fn connect(&mut self) -> Result<(), Error> {
        match self.stream {
            Some(_) => Err(Error::AlreadyConnected.into()),
            None => match TcpStream::connect(&self.remote_address).await {
                Ok(s) => {
                    self.stream = Some(s);
                    Ok(())
                }
                Err(_) => Err(Error::ConnectFailed.into()),
            },
        }
    }

    async fn send(&mut self, buff: &[u8]) -> Result<usize, Error> {
        let mut i = 0;
        return if let Some(stream) = &self.stream {
            loop {
                if std::result::Result::is_err(&stream.writable().await) {
                    return Err(Error::CheckConnection.into());
                }
                match stream.try_write(&buff[i..]) {
                    Ok(n) if n == buff.len() => {
                        return Ok(n);
                    }
                    Ok(n) => {
                        i += n;
                        continue;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        //continue;
                        return Err(Error::CheckConnection.into());
                    }
                    Err(_) => {
                        return Err(Error::CheckConnection.into());
                    }
                }
            }
        } else {
            Err(Error::NotConnected.into())
        };
    }

    async fn receive(&mut self, buff: &mut [u8]) -> Result<usize, Error> {
        if let Some(stream) = &self.stream {
            loop {
                if std::result::Result::is_err(&stream.readable().await) {
                    return Err(Error::CheckConnection.into());
                }
                match stream.try_read(buff) {
                    Ok(n) => {
                        return if 0 == n {
                            Err(Error::ConnectionClosed.into())
                        } else {
                            Ok(n)
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    _ => {
                        return Err(Error::ReceiveFailed.into());
                    }
                }
            }
        } else {
            Err(Error::CheckConnection.into())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::connection::TcpConnection;
    use crate::listener::TcpListener;
    use std::convert::TryFrom;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use tokio::runtime::Builder;
    use tokio::task;

    async fn client_worker(address: String) {
        let connection = TcpConnection::create(std::net::SocketAddr::from_str(&address).unwrap());
        let mut connection = connection.lock().await;
        let r = connection.connect().await;
        assert!(!r.is_err());
        for _i in 0u16..5 {
            let r = connection.send(b"ping").await;
            assert!(r.is_ok());
            let bytes = r.unwrap();
            assert_eq!(bytes, 4);

            let mut buff: [u8; 32] = [0; 32];
            let r = connection.receive(&mut buff).await;
            assert!(r.is_ok());
            let bytes = r.unwrap();
            assert_eq!(bytes, 4);
            assert_eq!(&buff[0..4], b"pong");
        }
        return;
    }

    async fn listen_worker(address: String) {
        {
            let r = TcpListener::create(std::net::SocketAddr::from_str(&address).unwrap()).await;
            assert!(r.is_ok());

            let listener = r.unwrap();
            let mut listener = listener.lock().await;
            let connection = listener.accept().await;
            assert!(connection.is_ok());

            let connection = connection.unwrap();
            let mut connection = connection.lock().await;
            for _i in 0u16..5 {
                let mut buff: [u8; 32] = [0; 32];
                let r = connection.receive(&mut buff).await;
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), 4);
                assert_eq!(&buff[0..4], b"ping");

                let r = connection.send(b"pong").await;
                assert!(r.is_ok());
                assert_eq!(r.unwrap(), 4);
            }
        }
    }

    async fn run_test(address: String) {
        let a1 = address.clone();
        let j1 = task::spawn(async {
            let f = listen_worker(a1);
            f.await;
            return;
        });

        let a2 = address.clone();
        let j2 = task::spawn(async {
            let f = client_worker(a2);
            f.await;
            return;
        });
        let (r1, r2) = tokio::join!(j1, j2);
        if r1.is_err() {
            assert!(false);
        }
        if r2.is_err() {
            assert!(false);
        }
    }

    #[test]
    pub fn ping_pong_multi_thread() {
        let runtime = Builder::new_multi_thread().enable_io().build().unwrap();

        runtime.block_on(async {
            run_test(String::from("127.0.0.1:4050")).await;
        });
    }

    #[test]
    pub fn ping_pong_single_thread() {
        let runtime = Builder::new_current_thread().enable_io().build().unwrap();

        runtime.block_on(async {
            run_test(String::from("127.0.0.1:4051")).await;
        });
    }
}
