use std::io;

fn main() {
    println!("guess the number");

    println!("please input  your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    println!("your guessed {guess}")
}
