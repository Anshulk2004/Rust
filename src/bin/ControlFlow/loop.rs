
fn main(){
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{
            break counter*2;
        }
    };
    println!("The result is {}",result);

    let a =[1,2,3,4,5];
    let mut index = 0;

    while index <5 {
        println!("Value is {}",a[index]);
        index = index+1;        
    }

    for element in a {
        println!("Value is {}",element);      
    }
    
    for number in (1..4).rev(){
        println!("Value is {}",number);
    }
}