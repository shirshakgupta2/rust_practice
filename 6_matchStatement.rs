fn main(){
    let num = 70;
    print!(" the number ");
    match num{
        //matches a single value
        0 => println!("yes it's zero"),
        1 => println!("yes it's one"),
        //matches several values
        3|4|5|6|7 => println!(" is between 3 and 7"),
        //match an inclusive range
        10..=60=> println!("range between 10 and 60"),
        //handle the rest of the range or say default
        _ => println!("is more then 60"),
        
    }
    println!(" the number is {}",num);
    
}