use console_interface::{CharPressedType, GuessType};

pub struct Guesser {
    pub wotd: String,
    pub ui: console_interface::ConsoleInterface,
}

impl Guesser {
    pub fn init_ui(&self) {
        self.ui.init();
    }

    fn get_user_guess(&self) -> String {
        let mut user_guess_input = String::new();
        let mut chars_entered = 0;
        let mut enter_pressed = false;

        while !enter_pressed || chars_entered != 5 {
            match self.ui.get_next_char_in_guess() {
                (CharPressedType::Valid, c) => {
                    if chars_entered < 5 {
                        self.ui.write_guess_input_char(c);

                        user_guess_input.push(c);
                        chars_entered += 1;
                    }
                }
                (CharPressedType::Enter, _) => {
                    if chars_entered == 5 {
                        enter_pressed = true;
                    }
                }
                (CharPressedType::Backspace, _) => {
                    if chars_entered > 0 {
                        self.ui.delete_last_char_in_guess();

                        user_guess_input.pop();
                        chars_entered -= 1;
                    }
                }
                (_, _) => {}
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

    fn tell_user_outcome(&self, guess: &String) {
        // build up vector of outputs, then pass to console interface
        let mut guess_outcomes: Vec<(char, GuessType)> = Vec::new();

        for (c1, c2) in guess.chars().zip(self.wotd.chars()) {
            if c1 != c2 {
                // does the char exist at all in the wotd
                if self.wotd.contains(c1) {
                    guess_outcomes.push((c1, GuessType::WrongPlace));
                } else {
                    guess_outcomes.push((c1, GuessType::Incorrect));
                }
            } else {
                guess_outcomes.push((c1, GuessType::Correct));
            }
        }

        self.ui.print_user_guess(guess_outcomes);
    }

    pub fn run_guessing_process(&self) -> bool {
        let user_guess = self.get_valid_guess();

        let is_correct = self.wotd.eq(&user_guess);

        self.tell_user_outcome(&user_guess);

        return is_correct;
    }
}
