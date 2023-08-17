use std::{
    thread,
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}, time::Duration
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(4);
        
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        //the following would create an unlimited number of threads and potentially overload the
        //system
        //thread::spawn(|| {
        //  handle_connection(stream);
        //  });
        }
    }
}
fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

