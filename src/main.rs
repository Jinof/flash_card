use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    str, thread,
};

fn main() {
    let address = "0.0.0.0:7878";
    println!("Use flash card to help your learning !!!");
    println!("flash card running at {}", &address);

    let listener = TcpListener::bind(&address).unwrap();

    let mut pool = Pool::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.handle_connection(stream);
            }
            Err(e) => {
                panic!("connection establish failed {}", e)
            }
        }
    }
}

struct Pool {}

impl Pool {
    fn new() -> Pool {
        Pool {}
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let child = thread::spawn(move || {
            let mut buf = [0; 1024];
            stream.read(&mut buf).unwrap();
            println!("{}", str::from_utf8(&buf).unwrap());
        });
        child.join().unwrap();
    }
}
