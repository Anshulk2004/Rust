//Using cargo run in terminal only will run the main.rs file and thus will we have to manually
// run the file from the run icon button.

// alternative to this is using the below variables.rs file as a module and include that module in the main.rs file.
// We have to pub to make the function public , thereby main.rs can access it otherwise it will not be able to access it.

pub fn testing_variables(){
    let x: i32 = -1;
    let y: u8 = 4;
    let z: f32 = 100.00;
    println!("{}",x);
    println!("{}",y);
    println!("The value of z is {}",z);

    let mut x = 5;
    println!("{}",x);

    x = 9;
    println!("{}",x);

    // constant only in uppercase
    const Y: u32 = 89;
    println!("{}",Y);
    
}

pub(crate) fn local_var(){
    let x = 5;
    let x = x+1;

    {
        let x = x*2;
        println!("The value of x is {}",x);
    }
    println!("{}",x);

    let spaces = "  ";
    let spaces = spaces.len();
    // both spaces have different types , thereby let function will allow shadowing.
    println!("{}",spaces);
}