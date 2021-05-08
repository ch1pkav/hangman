use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

fn main() {
    let mut word: String = "".to_string();
    let mut guessed_chars = Vec::new();
    let mut all_guesses = Vec::new();
    let mut strikes = 10;

    let file = File::open("words").unwrap();
    let mut reader = BufReader::new(file);
    for _i in 1..rand::thread_rng().gen_range(1..370102) {
        word = "".to_string();
        reader.read_line(&mut word).unwrap();
    }
    word = word.trim().to_string();
    let hidden_word = "_".repeat(word.chars().count());

    println!("{}", hidden_word);

    loop {
        print!("Please input your guess: ");


        io::stdout().flush().expect("Error flushing stdout.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let guess = guess.trim().to_string();

        if guess==word {
            println!("You won!");
            break;
        }

        if !all_guesses.contains(&guess) {
            all_guesses.push(guess.clone());
        }

        let mut hidden_word = String::new();

        match word.contains(&guess) && guess.chars().count() == 1 {
            true => {
                if !guessed_chars.contains(&guess) {
                    guessed_chars.push(guess);
                }
            }
            false => {
                strikes -= 1;
                println!("You get a strike! Strikes left: {}", &strikes);
            }
        }

        for ch in word.chars() {
            match guessed_chars.contains(&ch.to_string()) {
                true => {
                    hidden_word.push(ch);
                }
                false => hidden_word.push('_'),
            }
        }

        println!("Characters guessed: {:?}", all_guesses);
        println!("{}", hidden_word);

        if hidden_word == word {
            println!("You won!");
            break;
        }

        if strikes == 0 {
            println!("You lost! The word was {}", word);
            break;
        }
    }
}
