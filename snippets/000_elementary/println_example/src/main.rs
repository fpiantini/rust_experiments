// to print this Rectangle structure we have to 'opt-in'
// for the debugging information printout functionality
// using the #[derive(Debug)] annotation
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    // --------------------------------------------------------------
    // the '!' at the end of println, means MACRO
    println!("Hello, world!");

    // --------------------------------------------------------------
    let an_integer = 42;
    println!("an integer: {}", an_integer);

    // --------------------------------------------------------------
    // Putting the specifier :? inside the {} tells println!
    // we want to use an output format called Debug.
    // Debug is a trait that enables us to print out our struct
    // in a way that is useful for developers so we can see its
    // value while we’re debugging our code.
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("the months: {:?}", months);

    // --------------------------------------------------------------
    // this is possible only because we have used [#derive(Debug)]
    // when we have defined the struct Rectangle
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("a rectangle: {:?}", rect1);

}
