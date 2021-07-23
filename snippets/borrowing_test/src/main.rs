fn main() {

    let mut v = vec![10, 11];
    // The following sequence does not work:
    // let vptr = &v[1];   // <--- This is a immutable borrowing!
    // v.push(12);         // <--- This is a mutable borrowing, and cannot
                           //      cannot borrow 'v' as mutable because already
                           //      borrowed as immutable
    // println!("v[1] = {}", *vptr);

    // The following sequence does not work:
    // let vptr = &mut v[1];   // <--- This is a mutable borrowing!
    // v.push(12);             // <--- This is a mutable borrowing, and cannot
                               //      cannot borrow 'v' as mutable because already
                               //      borrowed
    // println!("v[1] = {}", *vptr);

    // For info:
    // rustc --explain E0502

    // This works:
    let vptr = &mut v[1];
    println!("v[1] = {}", *vptr);
    // Borrowing to 'vptr' ends here!
    v.push(12);
    // obviously this is not correct:
    // println!("v[1] = {}", *vptr);

    // But this works!
    let vptr = &mut v[2];   // <--- New borrowing
    println!("v[1] = {}", *vptr);
}
