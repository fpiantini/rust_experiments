// -------------------------------------------------
// This is to have formatting using {:?} for Point
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// -------------------------------------------------


// -------------------------------------------------
// don't using generics...
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// -------------------------------------------------
// using generics...
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// -------------------------------------------------

fn main() {

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'A', '0', '{', '~'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let float_number_list = vec![3.14, 7.23, 1.0, 42.0];
    let result = largest(&float_number_list);
    println!("The largest float is {}", result);

    let string_list = vec!["pippo", "pluto", "paperino"];
    let result = largest(&string_list);
    println!("The largest string is {}", result);

    let pnt = Point{x:3, y:5};
    println!("A point: {:?}", pnt);

    // This not compile. Point does not implement Copy and Compare traits
    //let point_list = vec![Point{x: 0, y:0}, Point{x: 2, y:7}, Point{x: 3, y:5}, Point{x: 2, y:9}, ];
    //let result = largest(&point_list);
    //println!("The largest point is {:?}", result);
}



