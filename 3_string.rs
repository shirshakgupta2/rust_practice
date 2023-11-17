fn main() {
    let name:&str="shirshak";
    println!("{}",name);
    let mut name_1=String::new();// this a empty string
    let name_2=String::from("shirshak");
    let name_3=String::from(name);
    println!("{}",name_1);
    println!("{}",name_2);
    println!("{}",name_3);
    println!("{}",name_3==(name_2));
    name_1+="hello how are you";
    name_1+=" hello how are you";
    println!("{}",name_1);
    name_1.push_str("hmm");
    println!("{}",name_1);

    
}

// fn another_function() {
//     println!("Another function.");
// }