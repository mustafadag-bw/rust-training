use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream}, fs, time::Duration, thread,
};

use web_server::ThreadPool;


fn handle_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./front-end/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./front-end/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./front-end/not-found.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// The CRLF(carriage return and line feed) sequence can also be written as \r\n, where \r is a carriage return and \n is a line feed
fn main() {
    // HTTP isn’t normally accepted on this port so our server is unlikely to conflict with any other web server you might have running on your machine
    // nonadministrators can listen only on ports higher than 1023
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");

}