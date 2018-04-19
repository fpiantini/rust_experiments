use std::io;

fn main() {

    println!("Please enter a sentence, I will convert it in pig latin:");

    let mut sentence = String::new();

    io::stdin().read_line(&mut sentence)
        .expect("Failed to read line");

    for word in sentence.split_whitespace() {
        print!("{} ", convert_to_piglatin(word));
    }
    println!("");

}

fn convert_to_piglatin(word: &str) -> String {

    let mut plword = String::new();
    let mut first_to_skip = true;

    // get the first character
    let ch = word.chars().next().unwrap();

    let is_wovel = is_wovel(ch);
    if is_wovel {
        first_to_skip = false;
    }

    for ch in word.chars() {
        if first_to_skip {
            first_to_skip = false;
        }
        else {
            plword.push(ch);
        }
    }

    if !is_wovel {
        plword.push('-');
        plword.push(ch);
        plword + "ay"
    }
    else {
        plword + "-hay"
    }
}

fn is_wovel(ch: char) -> bool {
    if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' ||
       ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U' ||
       ch == 'à' || ch == 'è' || ch == 'é' || ch == 'ì' || ch == 'ò' || ch == 'ù'
    {
        true
    }
    else {
        false
    }
}

