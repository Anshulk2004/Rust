use std::io;

fn main(){
    let a = [1,2,3,4,5];

    println!("Please Enter an Array Index");

    let mut index = String::new();
    
    io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

    let index: usize = match index.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("The given index is not a valid number.");
            return;
        }
    };  

    if index > a.len(){
        println!("Index is out of bounds.Enter a valid index from 0 to {}",a.len()-1);
        return;
    }

    let element = a[index];
    println!("The value of the element at that index {} is : {}",index , element);

}