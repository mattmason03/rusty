extern crate cards;

use cards::server::Server;

fn main() {
	let mut server = Server::new("127.0.0.1", 80);
	server.listen();
}