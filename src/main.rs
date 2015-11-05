use bufstream::BufStream;
use std::io::BufRead;
use std::io::Error as IoError;
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;
use self::smtp::{SmtpCommand, ParsingError};

mod smtp;

extern crate bufstream;

#[derive(Debug)]
enum ClientError {
    IoError(IoError),
    ParsingError(ParsingError),
}

impl From<IoError> for ClientError { fn from(error: IoError) -> ClientError { ClientError::IoError(error) } }
impl From<ParsingError> for ClientError { fn from(error: ParsingError) -> ClientError { ClientError::ParsingError(error) } }


fn handle_client(mut stream: BufStream<TcpStream>) -> Result<(), ClientError>{
    loop {
        let line = {
            let mut buffer = String::new();
            try!(stream.read_line(&mut buffer));
            buffer
        };

        let command = try!(SmtpCommand::from_str(line.trim()));

        println!("{:?}", command);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8005").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    match handle_client(BufStream::new(stream)) {
                        Ok(()) => println!("Connection ended"),
                        Err(err) => println!("Connection error: {:?}", err)
                    }
                });
            }
            Err(_) => { }
        }
    }
}
