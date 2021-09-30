use std::mem::size_of;
#[derive(Debug)]
struct VeryImportantMessage {
    message_type: u8,
    destination: u16,
    // + 1 bytes padding
    // total = 4 bytes = 32 bits
}
fn main() {
    let vim = VeryImportantMessage {message_type: 1, destination: 0x0302, };
    println!("VeryImportantStructure = {:?}", vim);
    println!(
        "VeryImportantMessage occupies {} bytes",
        size_of::<VeryImportantMessage>()
    );

    //let p = std::ptr::addr_of!(vim);
    //println!("Pointer to VeryImportantStructure = {:#?}", p[0]);

}
