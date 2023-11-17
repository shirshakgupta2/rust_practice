fn main() {
    let int_num_1:i128 = 12879098767657677766786548765547665 ;
     println!("{}",int_num_1);
    // let int_num_2:u128 = 121765776543897876878765567776468755465441;
    // println!("{}",int_num_2);
    let int_arc: isize = 10_000;
    println!("{}",int_arc);
    let nn:f32 = 12.2;
    let nnn:f64 = 12.22;
    println!("{}",nn);
    println!("{}",nnn);
    let var_bool: bool = false;
    println!("{}",var_bool);
    let var_char: char = '😁';
    println!("{}",var_char);

    let var_str: &str = "hello";
    println!("{}",var_str);

    let var_u8: u8 = 128;
    println!("{}",var_u8);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("{}",tup);
// can not directly print the tuple type of data 
    // let tup1 = (500, 6.4, 1);
    // println!("{}",tup1);


    //when we want to insert multiple values in  a single  variable
    let a:(i32,f32,bool,char,&str) = (20202,20.22,true,'a',"shirsha");
    println!("{:?}",a);
    println!("{:?}",a.0);
    println!("{:?}",a.1);
    
    
    let tup2 = (500, 6.4, 1);
    let(x,y,z)=tup2;
    println!(" the value of x is: {}",x);
    println!(" the value of y is: {}",y);
    println!("the value of y is:{}",z);

    
    
}