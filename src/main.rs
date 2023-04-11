use rand::Rng;
use std::fs;

fn main() {
    // Collect all possible words from file
    let data: String = fs::read_to_string("words.txt").expect("Unable to read file");
    let mut words: Vec<&str> = data.split("\n").collect();
    for i in words.iter_mut() {
        *i = i.trim();
    }
    loop {
        // A bunch of silly setup variables
        let word: String = String::from(words[rand::thread_rng().gen_range(0..words.len())]);
        let mut playing: bool = true;
        let mut lives: u8 = 8;
        let mut chars: Vec<String> = Vec::new();
        let mut input: String;
        let mut length: usize;
        // Game loop
        while playing {
            length = 0;
            input = String::new();
            display_game(&chars, &word, &lives);
            while length == 0 {
                length = std::io::stdin().read_line(&mut input).unwrap() - 2;
                // Add letter to guessed letters
                if length == 1 {
                    if !word.contains(&input.trim()) && !chars.contains(&input) {
                        lives -= 1;
                    }
                    if !chars.contains(&input) {
                        chars.push(input.clone());
                    }
                } else if length > 1 {
                    lives -= 1;
                }
                // Check for wins/losses
                if check_win(&word, &input, &chars) {
                    println!("You win! The word was {}", word);
                    playing = false;
                }
                if lives == 0 {
                    println!("You lose! The word was {}", word);
                    playing = false;
                }
            }
        }
    }
}

fn check_win(word: &String, input: &String, chars: &Vec<String>) -> bool {
    let mut temp: Vec<String> = Vec::new();
    if word == input.trim() {
        return true;
    }
    for i in word.chars() {
        for j in chars.iter() {
            if i.to_string() == *j.trim() {
                temp.push(j.trim().to_string());
            }
        }
    }
    *word == temp.join("")
}

fn display_game(chars: &Vec<String>, word: &String, lives: &u8) {
    let mut letters: Vec<String> = Vec::new();
    let mut guessed: bool;
    for i in word.chars() {
        guessed = false;
        for j in chars.iter() {
            if i.to_string() == *j.trim() {
                letters.push(j.trim().to_string());
                guessed = true;
            }
        }
        if !guessed {
            letters.push("_".to_string());
        }
    }
    println!("Lives: {}", lives);
    println!("{}", letters.join(""));
}
