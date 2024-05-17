fn main(){
    label_measure(5, 'c');    

    let y ={
        let x =3;
        x + 1
    };
    println!("The value of y is {}",y);
    // println!("The value of x is {}",x);
    let z = five();
    println!("Value : {}",z);

    let v = six(7);
    println!("Value : {}",v);

}

fn five() -> i32{
    5
}

fn six(x:usize) -> usize{
    x+1
}

fn label_measure(value: i32 , unit_label: char){
    println!("The value is {}{}",value,unit_label);
}

// Expressions do not include ending semicolons. 
// If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.