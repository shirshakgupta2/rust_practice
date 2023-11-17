//when we are looking for substring
fn main(){
    let name="Abhishek".to_string();
    let show=&name[0..4];

    println!("{}",show);
    println!("{}",name);
    let name_3="kumar";
    let name_4=String::from("shirhsak");
    let add=format!("{} {}",name_3,show);//can be used for strinig literal 
   
    let add_1=format!("{} {}",name_4,name);//can be used for strinig object
    let add_2=name+ &name_4;//direct concatenation can be used for string object only 

    println!("{}",add);
    println!("{}",add_1);
    println!("{}",add_2);
}