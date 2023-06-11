use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!(
        r###"
    ____  __ __    ___  _____ _____
    /    ||  |  |  /  _]/ ___// ___/
   |   __||  |  | /  [_(   \_(   \_ 
   |  |  ||  |  ||    _]\__  |\__  |
   |  |_ ||  :  ||   [_ /  \ |/  \ |
   |     ||     ||     |\    |\    |
   |___,_| \__,_||_____| \___| \___|
    THAT NUMBER!!!!               
    "###
    );
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 1;
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        print!("You guessed {guess}...");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("");
                win_banner(guess_count);
                break;
            }
        }
        guess_count += 1;
    }
}

fn win_banner(n: i32) {
    println!(
        r###"
        (`\ .-') /`            .-') _      .-') _   ('-.  _  .-')   
        `.( OO ),'           ( OO ) )    ( OO ) )_(  OO)( \( -O )  
     ,--./  .--.  ,-.-') ,--./ ,--,' ,--./ ,--,'(,------.,------.  
     |      |  |  |  |OO)|   \ |  |\ |   \ |  |\ |  .---'|   /`. ' 
     |  |   |  |, |  |  \|    \|  | )|    \|  | )|  |    |  /  | | 
     |  |.'.|  |_)|  |(_/|  .     |/ |  .     |/(|  '--. |  |_.' | 
     |         | ,|  |_.'|  |\    |  |  |\    |  |  .--' |  .  '.' 
     |   ,'.   |(_|  |   |  | \   |  |  | \   |  |  `---.|  |\  \  
     '--'   '--'  `--'   `--'  `--'  `--'  `--'  `------'`--' '--' 
    "###
    );

    if n > 12 {
        println!("): {n} were you trying?...");
    } else if n > 10 {
        println!("You'll get them next time... {n} tries... yikes");
    } else if n > 8 {
        println!("Good show with {n} tries...");
    } else if n >= 6 {
        println!("Wowie only {n} tries!");
    } else if n >= 4 {
        println!("No way!!! Only {n} tries?!");
    } else if n >= 3 {
        println!("You must actually have powers. Only {n} tries?!?!?!");
    } else if n > 1 {
        println!("You cheated, it is impossible to get it in {n} tries... CHEATER");
    }
}
