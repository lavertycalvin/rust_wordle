extern crate pancurses;
use pancurses::{endwin, initscr};

fn main() {
    let mut remaining_guesses = 6;
    let mut has_won = false;
    let mut word_of_the_day = "Hello".to_string();

    let console = console_interface::ConsoleInterface { window: initscr() };

    //word_dictionary::get_todays_word(&mut word_of_the_day);

    let guesser = guess::Guesser {
        wotd: word_of_the_day,
        ui: console,
    };

    guesser.init_ui();

    while !has_won && remaining_guesses > 0 {
        has_won = guesser.run_guessing_process();
        remaining_guesses -= 1;
    }

    if has_won {
        //println!("Congratulations!");
    } else {
        //println!("Better luck next time!");
    }

    endwin();
}
