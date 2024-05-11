use std::io;
// fn main() {
// println!("Welcome to the guess number game");
// println!("Enter the number");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed");

//     println!("You guessed the number: {guess}")
// }
fn main(){
    println!("Guess the number");
    println!("Enter the number you want to guess");;
    let mut num = String::new();
    io::stdin()
    .read_line(&mut num)
    .expect("Failed to get the number");

println!("This is the number you have guessed:{num}");


}