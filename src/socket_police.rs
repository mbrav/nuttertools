use clap::{arg, ArgAction, Args};
use std::net::{TcpListener, TcpStream};

use crate::Error;

/// Tool for spoofing red light activity on your ports and sockets
#[derive(Args)]
pub struct Options {
    /// Specify listen port number
    #[arg(short, long, value_name = "NUM", default_value_t = 8000)]
    port: u16,
    /// Specify port to forward connection
    #[arg(short, long, value_name = "NUM", default_value_t = 8001)]
    forward: u16,
    /// Specify whether to expose host to external connections
    #[arg(short, long, action=ArgAction::SetTrue)]
    open: bool,
}

fn handle_stream(stream: TcpStream) {
    println!("Stream: {stream:?}");
}

/// # Errors
///
/// Will return `Err` string is empty
pub fn main(opts: &Options) -> Result<(), Error> {
    let url = match &opts.open {
        false => format!("127.0.0.1:{}", &opts.port),
        true => format!("0.0.0.0:{}", &opts.port),
    };

    let listener = TcpListener::bind(&url);

    match listener {
        Ok(listener) => {
            println!("Server listening on {url}");
            for stream in listener.incoming() {
                handle_stream(stream.expect("Error unwrapping stream"));
            }
            Ok(())
        }
        Err(e) => {
            println!("Problem opening socket: {e:?}");
            Err(Error::ConnectionError)
        }
    }
}
