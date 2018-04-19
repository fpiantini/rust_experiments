use std::collections::HashMap;


fn main() {

    let mut members = HashMap::new();

    // Notare che senza le parentesi che delimitano i blocchi non funziona...
    // Per problemi di borrowing
    {
        let xxx = members.entry("engineering").or_insert(Vec::new());
        xxx.push("Francesco");
    }
    {
        let xxx = members.entry("engineering").or_insert(Vec::new());
        xxx.push("Vincenzo");
    }
    {
        let xxx = members.entry("sales").or_insert(Vec::new());
        xxx.push("Andrea");
    }

    println!("{:?}", members);



    //loop {
    //    io::stdin().read_line(&mut sentence)
    //        .expect("Failed to read line");
    //}
}
