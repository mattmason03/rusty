pub mod client;
pub mod server;

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
    	assert_eq!("Client", client::hello());
    	assert_eq!("Server", server::hello());
    }
}
