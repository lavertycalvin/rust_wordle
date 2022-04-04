extern crate pancurses;
use pancurses::{init_pair, noecho, start_color, use_default_colors, Input};

pub enum CharPressedType {
    Invalid,
    Valid,
    Newline,
    Backspace,
    Enter,
}

pub enum GuessType {
    Incorrect,
    Correct,
    WrongPlace,
}

enum ColorDefs {
    DefaultColorPair = 0,
    IncorrectColorPair = 1,
    WrongPlaceColorPair = 2,
    CorrectColorPair = 3,
}

pub struct ConsoleInterface {
    pub window: pancurses::Window,
}

impl ConsoleInterface {
    const DEFAULT_COLOR: i16 = -1;

    pub fn init(&self) {
        self.window.keypad(true);
        noecho();
        start_color();
        use_default_colors();

        self.make_color_pairs();

        self.print_header();

        self.window.refresh();
    }

    fn make_color_pairs(&self) {
        init_pair(
            ColorDefs::DefaultColorPair as i16,
            ConsoleInterface::DEFAULT_COLOR,
            ConsoleInterface::DEFAULT_COLOR,
        );
        init_pair(
            ColorDefs::IncorrectColorPair as i16,
            pancurses::COLOR_WHITE,
            ConsoleInterface::DEFAULT_COLOR,
        );
        init_pair(
            ColorDefs::WrongPlaceColorPair as i16,
            pancurses::COLOR_WHITE,
            pancurses::COLOR_YELLOW,
        );
        init_pair(
            ColorDefs::CorrectColorPair as i16,
            pancurses::COLOR_WHITE,
            pancurses::COLOR_GREEN,
        );
    }

    fn print_header(&self) {
        self.window
            .printw("Type in your guesses below, press Ctrl-C to quit\n");
        //self.window.printw(format!("Word of the day is: {}\n", self.wotd));
        self.window.printw(format!("GUESS\n"));
    }

    fn move_cursor_left(&self) {
        self.window
            .mv(self.window.get_cur_y(), self.window.get_cur_x() - 1);
    }

    pub fn move_beggining_of_ln(&self) {
        self.window.mv(self.window.get_cur_y(), 0);
    }

    fn write_colored_char(&self, output_chr: char, color_def: ColorDefs) {
        self.window.attron(pancurses::A_COLOR);
        self.window.color_set(color_def as i16);
        self.window.addch(output_chr);
        self.window.attroff(pancurses::A_COLOR);
    }

    pub fn get_next_char_in_guess(&self) -> (CharPressedType, char) {
        let pressed_type: CharPressedType;
        let mut ret_char = '\0';

        match self.window.getch() {
            Some(Input::Character(c)) => match c {
                '\n' => {
                    pressed_type = CharPressedType::Enter;
                    ret_char = '\n';
                }
                lower_c if lower_c.is_ascii_lowercase() => {
                    pressed_type = CharPressedType::Valid;
                    ret_char = lower_c;
                }
                _ => {
                    pressed_type = CharPressedType::Invalid;
                }
            },
            Some(Input::KeyBackspace) => {
                pressed_type = CharPressedType::Backspace;
            }
            _ => {
                pressed_type = CharPressedType::Invalid;
            }
        }
        return (pressed_type, ret_char);
    }

    pub fn delete_last_char_in_guess(&self) {
        self.move_cursor_left();
        self.window.delch();
        self.window.refresh();
    }

    pub fn write_guess_input_char(&self, c: char) {
        self.window.addch(c);
        self.window.refresh();
    }

    pub fn print_user_guess(&self, guess_vec: Vec<(char, GuessType)>) {
        for (c, guess_type) in guess_vec {
            match guess_type {
                GuessType::Correct => self.write_colored_char(c, ColorDefs::CorrectColorPair),
                GuessType::Incorrect => self.write_colored_char(c, ColorDefs::IncorrectColorPair),
                GuessType::WrongPlace => self.write_colored_char(c, ColorDefs::WrongPlaceColorPair)
            }
        }
        self.window.addch('\n');
        self.window.refresh();
    }
}
