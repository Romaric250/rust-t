// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// fn main() {
//     println!("this is a guessing game");
//     let secret_number = rand::thread_rng().gen_range(1..=100);


//     println!("The secret number is {secret_number}");



//     loop{
//         println!("Please enter your guess: ");
        
//     let mut guess = String::new();

//     io::stdin()
//     .read_line(&mut guess)
//     .expect("Faild to read the line, probably an error with the underlining operating systems");
//     // .expect("Failed to read the line");
//     // .expect("Faild to readline");

//     let guess:u32 = match guess.trim().parse(){
//         Ok(num) => num,
//         Err(_) => continue,
//     };

//         match guess.cmp(&secret_number){
//             Ordering::Less=> println!("The guess is small"),
//             Ordering::Greater=>println!("The guess is big"),
//             Ordering::Equal =>{ 
//             println!("You win the game!");
//             break;
//             }
//         }
//     }



//     // println!("Your guess is, {guess}");


//     // println!("Please input your guess number: ");
//     // println!("Hello, world!");
// }




fn main(){
    println!("this is a very good thing");
    let num = new_function(4.3,45.6);
    println!("The number you are talking about is this one i guess so {num}");
}


fn new_function(num1:f64, num2:f64) -> f64{
    return num1*num2;
}


fn is_greater(fx:f32, nx:f32) -> f32{
    println!("This is quit interesting.");
    return fx^nx; 

}