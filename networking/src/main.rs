// Import the functions and types of the TcpListener module into scope. Avoids
// having to type-qualify all the calls we make, i.e. instead of writing:
// - std::net::TcpListener::bind(...)
// we write:
// - TcpListener::bind(...)
//
// Importing std::io::prelude* is necessary to use the write/read functions of
// the TCP streams. Rust specifies that you need to import the traits which the
// types implement into scope before using the functions defined for such traits
//
// std::fs::File is used to open and manipule files on the filesystem
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

// The parameter type is mutable and copy. That means that the function takes
// ownership of "stream", and "stream" will go out of scope and be deleted
// when the function completes
fn handle_connection(mut stream: TcpStream) {
    // Create a slice of integers, inferring their type
    let mut buffer = [0; 512];

    // Read the incoming data into the buffer
    let buf_size = stream.read(&mut buffer).unwrap();

    println!("= Read {} bytes.", buf_size);

    // ::from_utf8_lossy() takes a chunk of bytes representing utf-8 encoded
    // unicode text and produces a string, replacing invalid utf-8 sequences
    // with the unicode replacement character �
    println!("= Request:\n{}\n", String::from_utf8_lossy(&buffer[..]));

    // Create a slice of raw bytes from a string by using "b" in front of the
    // string literal
    //
    // This represents a request requesting for the root page of the server
    let get_request = b"GET / HTTP/1.1\r\n";

    // This syntax allows us to return a tuple of values from the if expression
    // depending on which of the branches of the if expression was taken by the
    // code. Since the branches are expressions, these can be assigned to a
    // variable
    //
    // If the client requests the root page, we return our equivalent of
    // an index.html; if not, simply return a 404
    let (status_line, filename) = if buffer.starts_with(get_request) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello_rust.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    }; // The ; for the two variables which we are creating is here

    // ::open will create a File instance; it can be thought of as when calling
    // ::new on other types. (this might help some understand as it is somehow
    // more similar to using ::operator new() in C++
    //
    // We make the file mut because the traits function .read_to_string (from
    // the trait std::io::Read) uses a mut ref to self when calling the method
    let mut file_to_serve = File::open(filename).unwrap();

    // Store the contents of the file in a string; make it mut because it will
    // be filled later and not at creation
    let mut content = String::new();
    // Place the whole contents of the file, until EOF is reached, into the
    // String passed
    file_to_serve.read_to_string(&mut content).unwrap();

    // format!() is a macro which creates a value of type String by using the
    // the syntax provided in the first argument. It can be thought of as a
    // similar macro to print!() or println!() but instead of printing to the
    // STDOUT, it "prints" the results into a String, and then returns such
    // String.
    //
    // It automatically panics if the formatting trait implementation returns an
    // error
    let response = format!("{}{}", status_line, content);
    println!("= Response:\n{}\n", response);

    // .as_bytes returns a non-mutable reference to a byte slice containing the
    // byte representation of the String slice
    stream.write(response.as_bytes()).unwrap();
    // Flush the stream as it is buffered. Flushing means: "output all the data
    // (in this case text) which you have accmmulated from me using .write
    // on you".
    // If we did not do this, the data would be flushed at another point in time
    // and not right after the call to the above .write()
    stream.flush().unwrap();
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
    // which implements the ToSocketAddrs trait, and String slices do
    // Slices are a
    // convenient way of referring to portions of, in this case, Strings.
    // String literals are treated as String slices, that is "str"s and can
    // be thought of as a slice pointing to some section of the .text portion
    // of the binary
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

        println!("=== Connection established!\n");

        handle_connection(stream);

        println!("=== Closing connection.\n");
    }
}
