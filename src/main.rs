use ncurses::*;
use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_word() -> String {
    let mut word: String = "".to_string();
    let file = File::open("words").unwrap();
    let mut reader = BufReader::new(file);
    for _i in 1..rand::thread_rng().gen_range(1..370102) {
        word = "".to_string();
        reader.read_line(&mut word).unwrap();
    }
    word.trim().to_string()
}

fn main() {
    let mut guessed_chars = Vec::new();
    let mut all_guesses = Vec::new();
    let mut strikes = 10;
    let word = get_word();
    let mut hidden_word = "_".repeat(word.chars().count());

    initscr();
    raw();

    loop {
        clear();
        mvaddstr(0, 0, &hidden_word);
        mvaddstr(1, 0, format!("Strikes left: {}", &strikes).as_ref());
        mvaddstr(
            2,
            0,
            format!("Characters guessed: {:?}", all_guesses).as_ref(),
        );
        mvaddstr(3, 0, "Please input your guess: ");

        let mut guess = String::new();

        getstr(&mut guess);

        let guess = guess.trim().to_string();

        if guess == word {
            clear();
            addstr("You won!");
            break;
        }

        if !all_guesses.contains(&guess) {
            all_guesses.push(guess.clone());
        }

        hidden_word = "".to_string();

        match word.contains(&guess) && guess.chars().count() == 1 {
            true => {
                if !guessed_chars.contains(&guess) {
                    guessed_chars.push(guess);
                }
            }
            false => {
                strikes -= 1;
            }
        }

        for ch in word.chars() {
            if guessed_chars.contains(&ch.to_string()) {
                hidden_word.push(ch);
            } else {
                hidden_word.push('_');
            }
        }

        if hidden_word == word {
            clear();
            addstr("You won!");
            break;
        }

        if strikes == 0 {
            clear();
            addstr(format!("You lost! The word was {}", word).as_ref());
            break;
        }
    }
    getch();
    endwin();
}
