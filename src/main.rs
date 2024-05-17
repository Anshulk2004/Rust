// Doing the changes in main.rs file so as to include the variables.rs file as well.

mod variables;

fn main() {
    println!("Hello, world!");
    mainn();
    variables::testing_variables();
    variables::local_var();
}

fn mainn(){
    println!("Hello");
}