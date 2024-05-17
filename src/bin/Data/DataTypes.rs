fn main(){
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess is : {}",guess);

    // let guess: u8 = 256;
    // println!("guess : {}",guess);

    let x =  100.00;
    println!("{}",x);
    let x: f32 = 200.00;
    println!("{}",x);

    
}

// tore numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. 
