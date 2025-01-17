use std::io;
use rand::Rng;

fn str_to_int(x: &String)->i32{
    x.trim().parse().unwrap()
}

fn main(){
    let mut guess:String = String::new();
    // user input
    let _ = io::stdin().read_line(&mut guess);
    // random int set
    let x: i32 = rand::thread_rng().gen_range(1..=3);

    println!("Your Guess {}", guess);
    // type conversion & shadowing 
    let guess: i32 = str_to_int(&guess);

    if guess==x {
        println!("Your guess is right");
    }
    else {
        println!("Your guess {} is wrong! Right answer is {}", guess, x)
    }
}