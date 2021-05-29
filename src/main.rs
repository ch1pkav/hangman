use ncurses::*;
use rand::Rng;
use std::char;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_word_from_file() -> String {
    let file = File::open("words").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    lines
        .nth(rand::thread_rng().gen_range(0..370102))
        .expect("No line at that location")
        .expect("Failed to read line")
        .to_string()
}

fn get_guess(prompt_len: i32) -> String {
    let mut buffer = "".to_string();

    loop {
        let mut y: i32 = 0;
        let mut x: i32 = 0;
        let ch = getch();
        getyx(stdscr(), &mut y, &mut x);
        match ch {
            10 => break,
            127 => {
                if x > prompt_len {
                    mvaddstr(y, x - 1, " ");
                    mv(y, x - 1);
                    buffer.pop();
                }
            }
            65..=122 => {
                buffer.push(char::from_u32(ch as u32).expect("Invalid character!"));
                addch(ch as u32);
            }
            _ => {
                continue;
            }
        }
    }
    buffer.to_lowercase()
}

fn main() {
    let mut guessed_chars = Vec::new();
    let mut all_guesses = Vec::new();
    let mut strikes = 10;
    let word = get_word_from_file();
    let mut hidden_word = "_".repeat(word.chars().count());
    let prompt = "Please input your guess: ".to_string();

    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();

    loop {
        clear();
        mvaddstr(0, 0, &hidden_word);
        mvaddstr(1, 0, format!("Strikes left: {}", &strikes).as_ref());
        mvaddstr(
            2,
            0,
            format!("Characters guessed: {:?}", all_guesses).as_ref(),
        );
        mvaddstr(3, 0, &prompt);

        let guess = get_guess(prompt.chars().count() as i32);

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

        if word.contains(&guess) && guess.chars().count() == 1 {
            if !guessed_chars.contains(&guess) {
                guessed_chars.push(guess);
            }
        } else {
            strikes -= 1;
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
            addstr(format!("You won! The word was {}", word).as_ref());
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
