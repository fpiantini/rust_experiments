// On this example we define a set of 2-D objects
// for which is possible to compute the area
extern crate num_traits;

use num_traits::pow;

// ------------------------------------------------------
// THE TRAIT
// The following trait define a measurability
// characterist: an object is measurable if it
// is possible to compute the area and the perimeter
pub trait Measurable {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
// ------------------------------------------------------

// a triangle (not scalene)
#[derive(Debug)]
struct IsoTriangle {
    base: f64,
    height: f64,
}

// a rectangle
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// a circle
#[derive(Debug)]
struct Circle {
    radius: f64
}

// a polygon
#[derive(Debug)]
struct Polygon {
    segs: Vec<f64>
}

impl Measurable for IsoTriangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
    fn perimeter(&self) -> f64 {
        self.base + 2.0 * (pow((self.base/2.0), 2) + pow(self.height, 2)).sqrt()
    }
}

impl Measurable for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Measurable for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * 3.14159
    }
    fn perimeter(&self) -> f64 {
        2.0 * 3.14159 * self.radius
    }
}

impl Measurable for Polygon {
    // for polygon we are not able to compute the area!
    fn area(&self) -> f64 {
        panic!("Unable to compute area for polygon!");
    }
    fn perimeter(&self) -> f64 {
        let mut peri = 0.0;
        for seg in &self.segs {
            peri = peri + seg;
        }
        // serve??
        peri
    }
}

// -------------------------------------------
// Note that to avoid problems with borrowing & moving we have to define
// the following functions using references as parameters. See "the book"
// chapter 4.2
pub fn get_area<T: Measurable>(item: &T) -> f64 {
    item.area()
}

pub fn get_perimeter<T: Measurable>(item: &T) -> f64 {
    item.perimeter()
}
// -------------------------------------------


// -------------------------------------------------
fn main() {

    let triangle = IsoTriangle{base: 5.0, height: 10.0};
    let rect = Rectangle{width: 5.0, height: 10.0};
    let circle = Circle{radius: 5.0};
    let poly = Polygon{ segs: vec![1.0, 2.0, 3.0, 4.0]};

    // Note the different ways to get the values of perimeter and area
    println!("{:?} - Perimeter = {:?} - Area = {:?}", triangle, triangle.perimeter(), triangle.area());
    println!("{:?} - Perimeter = {:?} - Area = {:?}", rect, rect.perimeter(), rect.area());

    println!("{:?} - Perimeter = {:?} - Area = not able to compute", poly, poly.perimeter());
    // if we use the following line, the program will panic due to the call to poly.area()
    // println!("{:?} - Perimeter = {:?} - Area = {:?}", poly, poly.perimeter(), poly.area());

    println!("{:?} - Perimeter = {:?} - Area = {:?}", circle, get_perimeter(&circle), get_area(&circle));

}



