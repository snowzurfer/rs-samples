use std::fmt;
use std::ops;

// We generalise the Rectangle type we are creating so that different types
// can be used to specify the type of its attributes. In Rust we do so when
// declaring the struct, right after the name of the struct. The syntax is
// very similar to the one used to declare class templates in C++ but in rust
// we do not need to add a prelued with "template<typename T>".
//
struct Point<T> {
    x: T,
    y: T,
}

// We implement a trait for a type so that certain functions can be called on
// that type. In Rust we use traits to define what in C++ would be called
// interfaces: they specify operations that can be executed on types without
// coupling such operations to types. By implementing a trait for a type, we
// are saying that such type can now be used anywhere the code expects a type
// with a particular "interface" (still using the C++ terminology here because
// it makes sense to me, coming from a C++ background)
//
// The trait after T: specify that we require variables of type Point to be
// generalised over types that implement fmt::Display , that is
// writingo into an output stream. This way we can check at
// compile time that the types of the members are types which have traits we
// expect to use
impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.x)
    }
}

// Here we use the special syntax
// T: ops::Sub<Output = T>
// That is because the result of the operation self.x - rhs.x is of type
// <T as ops::Sub>::Output, whereas the Point type we are using expects a type
// T. Specifying that as a requirement makes the compiler use the right type (T)
impl<T: ops::Sub<Output = T>> ops::Sub for Point<T> {
    // Some traits expect you to define a type alias for their Output type
    type Output = Point<T>;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

fn main() {
    // First we create variables of type Point
    let a = Point{x: 12, y: 13};
    let b = Point{x: 2, y: 3};

    // Then we use the newly defined trait implementations
    let c = a - b;
    println!("Point c: {}", c);
}
