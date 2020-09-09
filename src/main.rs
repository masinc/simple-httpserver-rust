use anyhow::Result;
use itertools::Itertools;
use std::io::{prelude::*, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> Result<()> {
    // request
    {
        let reader = BufReader::new(&stream);
        let request: String = reader
            .lines()
            .take_while(|x| match x {
                Ok(x) => !x.is_empty(),
                Err(_) => false,
            })
            .map(|x| x.unwrap())
            .join("\n");

        println!("{}", request);
    }
    // response
    {
        let body = "Hello World!";
        // let now = Utc::now();
        let header = [
            "HTTP/1.1 200 OK",
            "Content-Type: text/plain",
            format!("Content-Length: {}", body.len()).as_str(),
        ]
        .join("\n");

        let mut writer = BufWriter::new(&stream);
        writeln!(writer, "{}\n\n{}", header, body)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
