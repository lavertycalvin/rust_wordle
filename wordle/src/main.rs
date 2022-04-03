fn main() {
    let mut remaining = 6;
    let mut has_won = false;

    loop {
        if remaining == 0 {
            break;
        }
        
        println!("Remaining guesses: {}", remaining);

    }

    if has_won {
        println!("Congratulations!");
    } else {
        println!("Better luck next time!");
    }
}

