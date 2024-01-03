use std::io ; 
use std::cmp::Ordering;
use rand::Rng ;
fn main() {
    println!("guess the number");
    println!("enter your guessing number");

    
    let secretNumber = rand::thread_rng().gen_range(1..=10);

    
    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read lines");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("you guessed number {}", guess);



        match guess.cmp(&secretNumber) {
        Ordering::Less => println!("Too small! "),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
                println!("You win!");
                break;
        }
    }
 }
}