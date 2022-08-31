fn main() {
    let s = String::from("book");

    // Add code here to call pluralize function
    // let plural_s = pluralize(&mut s.clone());
    // let plural_s = pluralize(s.clone());
    let plural_s = pluralize(&s);

    println!("I have one {}, you have two {}",
             s,
             plural_s,
    );
}

// fn pluralize(s:String) -> String{
// fn pluralize(s:&mut String) -> String{
fn pluralize(s:&str) -> String{
    // s.push_str("s");
    // s.to_string()
    // s + "s"
    s.to_owned() + "s"
}

