fn main(){
    label_measure(5, 'c');    
}

fn label_measure(value: i32 , unit_label: char){
    println!("The value is {}{}",value,unit_label);
}