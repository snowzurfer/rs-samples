
// Use this syntax to import more than one type from the same root type
use std::io::{self, Write};

fn fib(n: usize) -> usize {
    // Compared to C++, in Rust parenthesis around the expression for if/else
    // statements are not necessary. You can use them but the compiler will
    // warn you that they are unnecessary
    if n == 0 {
        // Numeric literals in Rust can be postfixed with the exact name of the
        // available numeric types, e.g. 1u32 or 2i64
        return 0usize;
    }
    else if n == 1 {
        // Returning values in Rust is also done by writing an expression
        // rather than
        // a statement. I.e. you can omit the trailing ";" at the end of the
        // expression, e.g.:
        //
        //   1usize
        //
        return 1usize;
    }
    else {
        return fib(n-1usize) + fib(n-2usize);
    }
}

fn main() {
    // "print!" is like "println!" which is used in most examples in the rust
    // book but it doesn't output a newline
    print!("Enter what Fibonacci number you want to compute: ");
    // We need to flush the buffer because print!() does not output a newline
    // character and does not flush; it uses STDOUT which is buffered
    io::stdout().flush().unwrap();

    // ::new() is an associated function implemented on a type, in this case on
    // type String, rather than an instance of String. They are basically
    // equivalent to C++'s static methods
    //
    // Making the variable "mut" (mutable) allows us to change it. In Rust
    // variables are constant (unmutable) by default
    let mut n = String::new();

    // stdin() returns an instance of std::io::Stdin, which represents a handle
    // to the STD output. Equivalent to C++'s std::cin
    //
    // .read_line() is used to read until a newline character is found
    //
    // .expect() checks the return type of .read_line(), which is an enum, and
    // uses the match control operator to check whether the function returned
    // an error enum variant or a success enum variant. On success, it returns
    // the value which the function returns upon success, whereas upon failure
    // it will panic and exit the program after printing the message we pased to
    // it.
    //
    // See enums in the Rust book and std::result, std::result::Result for more
    // information
    io::stdin().read_line(&mut n).expect("Failed to read from STDIN!");

    // In Rust we can perform variable "shadowing". Here the previous
    // declaration of "n" was of type String but now we declare a new variable
    // with name "n" but of numerical type, which is obtained by running the
    // .parse() method. When referring to "n" we will use the latest instance
    // of a type named "n" and not the String instance anymore.
    //
    // .parse() uses the trait FromStr to determine whether
    // a type supports this operation (converting to something from a String)
    let n: usize = n.trim().parse().expect("Invalid input!");

    println!("The computed value is: {}", fib(n));
}
