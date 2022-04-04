extern crate pancurses;
use pancurses::{Input, init_pair};

pub struct Guesser {
    pub wotd: String,
    pub output_window: pancurses::Window,
}

impl Guesser {
    const DEFAULT_COLOR: i16 = 0;
    const INCORRECT_COLOR: i16 = 1;
    const WRONG_PLACE_COLOR: i16 = 2;
    const CORRECT_GUESS_COLOR: i16 = 3; 


    pub fn make_color_pairs(&self) {
        init_pair(Guesser::DEFAULT_COLOR, -1, -1);
        init_pair(Guesser::INCORRECT_COLOR, pancurses::COLOR_WHITE, -1);
        init_pair(Guesser::WRONG_PLACE_COLOR, pancurses::COLOR_WHITE, pancurses::COLOR_YELLOW);
        init_pair(Guesser::CORRECT_GUESS_COLOR, pancurses::COLOR_WHITE, pancurses::COLOR_GREEN);
    }

    fn move_cursor_left(&self) {
        self.output_window.mv(
            self.output_window.get_cur_y(),
            self.output_window.get_cur_x() - 1,
        );
    }

    fn move_beggining_of_ln(&self) {
        self.output_window.mv(self.output_window.get_cur_y(), 0);
    }

    fn write_colored_char(&self, output_chr: char, color_pair: i16) {
        self.output_window.attron(pancurses::A_COLOR);
        self.output_window.color_set(color_pair);
        self.output_window.addch(output_chr);
        self.output_window.attroff(pancurses::A_COLOR);
        self.output_window.refresh();
    }

    fn get_user_guess(&self) -> String {
        let mut user_guess_input = String::new();
        let mut chars_entered = 0;
        let mut enter_pressed = false;

        while !enter_pressed || chars_entered != 5 {
            match self.output_window.getch() {
                Some(Input::Character(c)) => match c {
                    '\n' => {
                        if chars_entered == 5 {
                            enter_pressed = true;
                        }
                    }
                    _ => {
                        if chars_entered < 5 && c.is_ascii_lowercase() {
                            self.output_window.addch(c);
                            self.output_window.refresh();
                            user_guess_input.push(c);
                            chars_entered += 1;
                        }
                    }
                },
                Some(Input::KeyBackspace) => {
                    if chars_entered > 0 {
                        self.move_cursor_left();
                        self.output_window.delch();
                        self.output_window.refresh();
                        user_guess_input.pop();
                        chars_entered -= 1;
                    }
                }
                None => (),
                _ => {}
            }
        }
        return user_guess_input;
    }

    fn get_valid_guess(&self) -> String {
        let mut possible_guess = String::new();
        let mut is_valid = false;
        while !is_valid {
            possible_guess = self.get_user_guess();
            //TODO: same guess? hard mode?
            is_valid = self.is_valid_guess(&possible_guess);
        }
        return possible_guess;
    }

    fn is_valid_guess(&self, guess: &String) -> bool {
        let is_valid = true;

        //TODO: guessed before, hard guess, duplicates, in dict, etc...
        return is_valid;
    }

    fn print_guess_outcome(&self, guess: &String) {
        self.move_beggining_of_ln();
        for (c1, c2) in guess.chars().zip(self.wotd.chars()) {
            if c1 != c2 {
                // does the char exist at all in the wotd
                if self.wotd.contains(c1) {
                    self.write_colored_char(c1, Guesser::INCORRECT_COLOR);
                } else {
                    self.write_colored_char(c1, Guesser::WRONG_PLACE_COLOR);
                }
            } else {
                self.write_colored_char(c1, Guesser::CORRECT_GUESS_COLOR);
            }
        }

        self.output_window.color_set(Guesser::DEFAULT_COLOR);
        self.output_window.refresh();
    }

    pub fn run_guessing_process(&self) -> bool {
        let user_guess = self.get_valid_guess();

        let is_correct = self.wotd.eq(&user_guess);

        self.print_guess_outcome(&user_guess);

        self.output_window.addch('\n');
        self.output_window.refresh();

        return is_correct;
    }
}
