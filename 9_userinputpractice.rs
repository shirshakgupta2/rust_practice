fn main () {
    let mut num1 = String::new();
    println!("Enter Here");
    let _b1 = std::io::stdin().read_line(&mut num1).unwrap();
    let _x: i32 =num1.trim().parse().expect("Input not an integer");
   // println!("{}",_x);

    let mut num2 = String::new();
    println!("Enter Here");
    let _c1 = std::io::stdin().read_line(&mut num2).unwrap();
    let _y: i32 =num2.trim().parse().expect("Input not an integer");
    //println!("{}",_x);

    let mut num3 = String::new();
    println!("Enter Here");
    let _d1 = std::io::stdin().read_line(&mut num3).unwrap();
    let _z: i32 =num3.trim().parse().expect("Input not an integer");
   // println!("{}",_x);

   println!("{} {} {}", _x ,_y ,_z);
   println!("The sum of numbers are:{} ", _x +_y+_z);    
   


   
}