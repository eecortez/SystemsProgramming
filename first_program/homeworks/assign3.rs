fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let mut secret: i32 = 7;
    let mut guess: i32 = 0;
    let mut attempts: i32 = 0;

    loop {
        attempts = attempts + 1;

        if attempts == 1 {
            guess = 3;
        } else if attempts == 2 {
            guess = 10;
        } else {
            guess = 7;
        }

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("{} is correct!", guess);
            break;
        } else if result == 1 {
            println!("{} is too high.", guess);
        } else {
            println!("{} is too low.", guess);
        }
    }

    println!("It took {} guesses.", attempts);
}
