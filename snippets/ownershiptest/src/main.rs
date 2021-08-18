#[derive(Debug)]
struct Person { name: String, birth: u32 }

fn main() {
    let c = Person { name: "Palestrina".to_string(), birth: 1525};
    move_composer(c);
    // c is moved to move_composer() function,
    // after this operation c is unitialized,
    // so the following code is incorrect:
    // println!("Composer from main(): {:?}", c);
    // |
    // 5  |     let c = Person { name: "Palestrina".to_string(), birth: 1525};
    //    |         - move occurs because `c` has type `Person`, which does not implement the `Copy` trait
    // 6  |     move_composer(c);
    //    |                   - value moved here
    // ...
    // 10 |     println!("Composer from main(): {:?}", c);
    //    |                                            ^ value borrowed here after move

    let c = Person { name: "Dowland".to_string(), birth: 1563};
    borrow_composer(&c);
    // After the function, c returns to own c and its value
    // so the next is correct:
    println!("Composer from main(): {:?}", c);

    vector_moves_in_loop();
    vector_borrow_in_loop();

}

fn move_composer(p: Person) -> () {
    println!("Composer: {:?}", p);
}

fn borrow_composer(p: &Person) -> () {
    println!("Composer: {:?}", p);
}

fn vector_moves_in_loop() {
    let v = vec![
        "aaa".to_string(),
        "bbb".to_string(),
        "ccc".to_string(),
    ];
    println!("v before loop = [{}, {}, {}]", v[0], v[1], v[2]);

    // this is equivalent to:
    // for mut s in v.into_iter() {
    for mut s in v {
            s.push('!');
        println!("v element after modification = {}", s);
    }

    // v was moved in the previous for loop, so the following in not correct:
    // println!("v after loop  = [{}, {}, {}]", v[0], v[1], v[2]);

}

fn vector_borrow_in_loop() {
    let w = vec![
        "xxx".to_string(),
        "yyy".to_string(),
        "zzz".to_string(),
    ];
    println!("w before loop = [{}, {}, {}]", w[0], w[1], w[2]);

    for s in &w {
        // This cannot be done this time:
        // s.push('!');
        println!("w element in the loop (no mod) = {}", s);
    }

    // w was borrowed, so here it is valid again:
    println!("w after loop  = [{}, {}, {}]", w[0], w[1], w[2]);
}