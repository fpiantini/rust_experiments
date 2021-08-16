// Serde is a framework for serializing and deserializing
// Rust data structures efficiently and generically.
//
// The Serde ecosystem consists of data structures that know how to
// serialize and deserialize themselves along with data formats that
// know how to serialize and deserialize other things. Serde provides
// the layer by which these two groups interact with each other,
// allowing any supported data structure to be serialized and
// deserialized using any supported data format.
//
// Info:
// https://serde.rs/

use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
struct MyParameters {
    an_int: u64,
    another_int: u64,
    a_string: String,
}

fn main() {
    let pars = MyParameters {
        an_int: 10,
        another_int: 20,
        a_string: "pippo".to_string(),
    };
    println!("MyParameters = {:?}", pars);

    // 1. convert the MyParameters structure to JSON using serde_json
    let jstr = serde_json::to_string(&pars).expect("unable to serialize");
    println!("MyParameters serialized as JSON = {}", jstr);

    // 2. builds a json block and deserialize it on a MyParameters structure
    let x: u64 = 42;
    let y: u64 = 123456;
    let name = "Pluto";
    let jstr2 = serde_json::json!({
        "an_int": x,
        "another_int": y,
        "a_string": name,
    });
    println!("Other JSON data = {}", jstr2);
    let pars2: MyParameters = serde_json::from_str(&jstr2.to_string()).expect("unable to deserialize");
    println!("MyParameters deserialized from JSON = {:?}", pars2);
}
