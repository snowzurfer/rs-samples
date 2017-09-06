// Import the functions and types of the TcpListener module into scope. Avoids
// having to type-qualify all the calls we make, i.e. instead of writing:
// - std::net::TcpListener::bind(...)
// we write:
// - TcpListener::bind(...)
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// The parameter type is mutable and copy. That means that the function takes
// ownership of "stream", and "stream" will go out of scope and be deleted
// when the function completes
fn handle_connection(mut stream: TcpStream) {
    // Create a slice of integers, inferring their type
    let mut buffer = [0; 512];

    // Read the incoming data into the buffer
    let buf_size = stream.read(&mut buffer).unwrap();

    println!("Read {} bytes.", buf_size);

    // ::from_utf8_lossy() takes a chunk of bytes representing utf-8 encoded
    // unicode text and produces a string, replacing invalid utf-8 sequences
    // with the unicode replacement character �
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


}

fn main() {
    // Calling unwrap() causes the returned std::io::Result enum to be pattern
    // matched and
    // - in case of error, panic
    //   * Note: in a real-world situation we would handle the error by pattern
    //           matching its value and acting depending on it
    // - in case of success, return the value contained in the Ok variant, which
    //   in this case is a TcpListener instance
    //
    // It is possible to just pass a string because ::bind() accepts any type
    // which implements the ToSocketAddrs trait, and strings do
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Bound TCP listener socket at {} on port {}.\nListening...",
             listener.local_addr().unwrap().ip(),
             listener.local_addr().unwrap().port());

    // .incoming() returns an iterator of type std::net::Incoming which
    // implements the required interface for the trait, .next(&mut self).
    // Calling .next() (implicitly called by the for loop) on the iterator
    // accepts another connection
    for stream in listener.incoming() {
        // Retrieve the stream from the iterator's result, which is (the result)
        // of type enum Option<T>. As usual, .unwrap() will panic if the variant
        // type is None
        //
        // stream will be of type TcpStream; this type represents a connection
        // between the host and the client and can be used to write to/read from
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);

        println!("Closing connection.");
    }
}
