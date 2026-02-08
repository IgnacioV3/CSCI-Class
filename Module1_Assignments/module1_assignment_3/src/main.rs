//Function to check for correct guess
fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret {
        0
    }
    else if guess > secret {
        1
    }
    else {
        -1
    }
}

fn main() {

    //Secret, guess, and guess count variables for while loop
    let secret:i32 = 7;
    let mut guess:i32 = 3;
    let mut guess_count:i32 = 0;

    //While loop iterates and runs until guess == secret
    while guess != secret {
        guess_count += 1;

        let guess_result = check_guess(guess, secret);

        //If else statement checks if guess too high/low
        if guess_result == 1 {
            println!("Guess {} too High", guess);
        }
        else {
            println!("Guess {} too Low", guess);
        }
        
        guess += 1;

    }

    //Final guess count
    guess_count += 1;

    println!("\nGuess {} is correct!", guess);
    println!("It took you {} guesses!", guess_count);

}
