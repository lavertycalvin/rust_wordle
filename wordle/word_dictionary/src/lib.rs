#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn find_word_in_dict() -> String {
    return "Hello".to_string();
}

pub fn get_todays_word(word : &mut String) {
    let dict_word = find_word_in_dict();
    word.push_str(&dict_word);
}


