fn main() {
    let name = "Shirshak is my name".to_string();
    println!("Hello, {}!", name);
    let l=name.len();
    println!("Hello, length f string is {l}!",);
    let replace=name.replace("shirshak","what" );
    println!("Hello, string after replace is {replace}!",);
    

   // let  name1 = "Shirshak is my name".to_string();// push can be only used whenstring is mutable
    let mut name1 = "Shirshak is my name".to_string();
    name1.push('?');
    println!("Hello, string after push is {name1}",);

    name1.push_str("Aren't you sure ");
    println!("Hello, string after push_str is {name1}",);

}
