use rand::Rng;
use std::fs;

fn main() {
    let data = fs::read_to_string("words.txt").expect("Unable to read file");
    let mut words: Vec<&str> = data.split("\n").collect();
    words.pop();
    for i in words.iter_mut() {
        *i = i.trim();
    }
    println!("{:?}", words[rand::thread_rng().gen_range(0..2)]);
}
