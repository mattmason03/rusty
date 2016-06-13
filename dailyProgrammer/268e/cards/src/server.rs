use std::net::{SocketAddrV4, Ipv4Addr, TcpListener, TcpStream};
use std::thread;

pub fn hello() -> String {
	"Server".to_string()
}

pub struct Server {
	pub ip: String,
	pub port: u16,
	listener: Option<TcpListener>
}

impl Server {
	pub fn new(ip: &str, port: u16) -> Server {
		Server {
			ip: ip.to_string(),
			port: port,
			listener: None
		}
	}

	pub fn listen(&mut self) {
		let address = format!("{}:{}", self.ip, self.port);
		// let socketAddr = SocketAddrV4::new(self.ip, self.port);
		self.listener = Some(TcpListener::bind(&address[..]).unwrap());
		println!("{}", address);
	}

}