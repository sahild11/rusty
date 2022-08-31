use std::collections::HashMap;

struct Player{
    score: u8
}

impl Player{
    pub fn new() -> Self{
        Player {score:0}
    }
    pub fn score(&self) -> u8{
        self.score
    }
    pub fn set_score(&mut self, new_score:u8){
        self.score = new_score;
    }
}
fn main() {
    let thetext = String::from("this is the same as this");

    let mut counter = HashMap::new();

    for word in thetext.split_whitespace(){
        *counter.entry(word).or_insert(0) += 1;
    }
    println!("Word frequencies: {:#?}", counter);

    // Player score - use immutable reference within mutable reference scope
    let mut theplayer = Player::new();
    theplayer.set_score(theplayer.score()+1);
    println!("Score is now: {}", theplayer.score());
}
