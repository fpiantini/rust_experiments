use std::io::Write;

fn write_using_trait_object(writer: &mut dyn Write) -> std::io::Result<usize>
{
    // this function can be used with trait obiect "extracted" by different types
    writer.write(b", I'm a print inside a function that takes a trait object")
}

fn generic_write<W: Write>(out: &mut W) -> std::io::Result<()>
{
    out.write_all(b"Hello, I'm a print inside a generic write method")?;
    out.flush()
}

fn vector_to_string(v: &Vec<u8>) -> String {
    let mut str: String = "".to_string();
    for c in v {
        str.push(*c as char);
    }
    str
}


fn main() {

    let mut vec1: Vec<i32> = vec![];
    vec1.push(42);
    vec1.push(111);
    println!("vec1 = {:?}", vec1);

    // This is a compile time error: Vec<i32> does not implement Write trait
    // vec1.write(b"hello").expect("error writing");

    // Vec<u8> supports the Write trait, but not other types of Vecs
    // b"<text>" is a byte string (a slice of u8 values)
    let mut vec2: Vec<u8> = vec![];
    vec2.write(b"hello").expect("error writing");
    vec2.push(32);  // 32 = ' '

    let writer: &mut dyn Write = &mut vec2;
    writer.write(b"world").expect("error writing with trait object");
    write_using_trait_object(writer).expect("error writing with trait object passed to function");
    // this is wrong: writer is a trait object
    // that has access only to method of Write trait
    // writer.push(33); // 33 = '!'
    // This is instead obviously correct:
    vec2.push(33); // 33 = '!'
    println!("vec2 = {}", vector_to_string(&vec2));

    // Rust automatically converts ordinary references into trait objects when needed
    // so the following is correct:
    let mut vec3: Vec<u8> = vec![];
    vec3.write(b"hello").expect("error writing");
    write_using_trait_object(&mut vec3).expect("error writing with trait object passed to function");
    // in this case vec3 implements the Write trait, so rust performs conversion implicitily
    println!("vec3 = {}", vector_to_string(&vec3));

    // Polimorphism using generic function
    let mut vec4: Vec<u8> = vec![];
    generic_write(&mut vec4).expect("error writing with generic method");
    println!("vec4 = {}", vector_to_string(&vec4));

}
