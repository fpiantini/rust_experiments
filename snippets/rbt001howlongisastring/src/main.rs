// Remember that:
// pub struct String {
//     vec: Vec<u8>,
// }

const HELLO_WORLD: &'static str = "Hello World";
const HELLO_WORLD_R: &'static str = "Привет мир";
fn main() {
    println!("{} is {} bytes long.", HELLO_WORLD, HELLO_WORLD.len());
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.chars().count());
    println!("{} is {} bytes long.", HELLO_WORLD_R, HELLO_WORLD_R.len());
    println!("{} is {} characters long.", HELLO_WORLD_R, HELLO_WORLD_R.chars().count());
}
