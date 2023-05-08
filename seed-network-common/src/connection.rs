use tokio::{io::BufWriter, net::TcpStream};

pub struct Connection {
    _stream: BufWriter<TcpStream>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            _stream: BufWriter::new(stream),
        }
    }
}
