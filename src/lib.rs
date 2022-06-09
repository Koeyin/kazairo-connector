mod err;

use std::io::Write;
use std::net::{TcpListener, TcpStream};

struct EasySend {
    stream: Option<TcpStream>,
    addr: String,
}

impl EasySend {
    // Create a new EasySend object.
    pub fn new(address: &str) -> EasySend {
        EasySend {
            stream: None,
            addr: address.to_string(),
        }
    }

    // connect to specific address
    pub fn connect(&mut self) -> Result<(), String> {
        let stream = TcpStream::connect(&self.addr);
        match stream {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    // disconnect from server
    pub fn disconnect(&mut self) -> Result<(), String> {
        match self.stream {
            Some(ref mut stream) => {
                match stream.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => {
                        self.stream = None;
                        Ok(())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            None => Err("No stream to disconnect".to_string()),
        }
    }

    // send data to the stream
    pub fn send(&mut self, data: &str) -> Result<(), String> {
        match self.stream {
            Some(ref mut stream) => {
                match stream.write(data.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            }
            None => Err("No stream to send data".to_string()),
        }
    }
}

impl Drop for EasySend {
    fn drop(&mut self) {
        match self.stream {
            Some(ref mut stream) => {
                match stream.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                }
            }
            None => {}
        }
    }
}

struct EasyReceive {
    stream: Option<TcpStream>,
    addr: String,
}

impl EasyReceive{

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let error = super::err::Error::new(super::err::Recoverable::Recoverable, "Test error");
        error.solve();
    }
}
