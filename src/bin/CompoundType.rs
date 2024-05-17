fn main(){
    let tup: (i32,f64,u8) = (500,8.0,1);
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("The values of a,b,c are {} ,  {} ,  {}",a,b,c);

    let tup1 = ("a",400,78.0,true);
    let (x,y,z,w) = tup1;
    println!("Value of x : {}",x);
    println!("Value of y : {}",y);
    println!("Value of z : {}",z);
    println!("Value of w : {}",w);

    let a: [i32;5] = [3;5];
    let first = a[0];
    let second = a[1];
    let third = a[2];
    let fourth = a[3];
    let fifth = a[4];

    println!("Value of first is {}",first);
    println!("Value of second is {}",second);
    println!("Value of third is {}",third);
    println!("Value of fourth is {}",fourth);
    println!("Value of fifth is {}",fifth);
}