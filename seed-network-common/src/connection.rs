use tokio::{io::BufWriter, net::TcpStream};

pub struct Connection {
    stream: BufWriter<TcpStream>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(stream),
        }
    }
}
