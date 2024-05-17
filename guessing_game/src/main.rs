use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("*Guess the Number* ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {secret_number}");

    loop{
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
          
        // As the input is in string , we have to convert it into unsigned integer so as to compare with secret_number.   
        // let guess: u32 = guess.trim().parse().expect("Please enter a Valid Number!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>continue,
        };
        // expect function easiers the code , but terminates the function if non digit number is taken as input
        // The above match function with Ok and Err , the two outputs , uses the arm rule , compares with both the results
        // Executes whichever is correct , thereby if typo is there , it continues the program and gives another chance

        println!("You guessed : {guess}");       

        match guess.cmp(&secret_number){
                Ordering::Less => println!("Too Small"),
                Ordering::Greater => println!("Too Big"),
                Ordering::Equal => {
                    println!("You win");
                    break;
                }
            }       
        } 
    }

    // match uses arm function , compares the result with every equation/arm written below , and
    // whichever is right , it executes the statement.
    // Detailed explaination through ChatGPT