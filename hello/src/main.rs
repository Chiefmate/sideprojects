// use std::fs::File;
use std::{
	fs,
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
	thread,
};
use hello::ThreadPool;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(4);

	for stream in listener.incoming().take(2) {
		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
	}
	println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
	// let mut buffer = [0; 512];
	// stream.read(&mut buffer).unwrap();
	let buf_reader = BufReader::new(&mut stream);

	let request_line = buf_reader.lines().next().unwrap().unwrap();

	let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
		("HTTP/1.1 200 OK", "hello.html")
	} else {
		("HTTP/1.1 404 NOT FOUND", "404.html")
	};
	let contents = fs::read_to_string(filename).unwrap();
	let response = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}",
		status_line,
		contents.len(),
		contents
	);
	// stream.write(response.as_bytes()).unwrap();
	// stream.flush().unwrap();
	stream.write_all(response.as_bytes()).unwrap();
}
