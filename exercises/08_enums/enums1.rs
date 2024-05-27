// enums1.rs
//
// No hints this time! ;)

// I AM DONE

#[derive(Debug)]
enum Message {
    Quit,// "why",
    Echo,//: "do",
    Move,//: "I",
    ChangeColor,//: "try",
}
//easier then I was expecting. Its weird we can print that. means theres meta
//data that knows that. 

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
