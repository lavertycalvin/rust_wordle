extern crate pancurses;
use pancurses::{endwin, initscr, noecho, start_color, use_default_colors};

fn main() {
    let mut remaining_guesses = 6;
    let mut has_won = false;
    let mut word_of_the_day = String::new();

    let window = initscr();

    window.keypad(true);
    noecho();
    window.refresh();
    start_color();
    use_default_colors();



    word_dictionary::get_todays_word(&mut word_of_the_day);

    window.printw("Type in your guesses below, press Ctrl-C to quit\n");
    window.printw(format!("Word of the day is: {}\n", word_of_the_day));
    window.printw(format!("GUESS\n"));

    let guesser = guess::Guesser {
        wotd: word_of_the_day,
        output_window: window,
    };
    guesser.make_color_pairs();

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
