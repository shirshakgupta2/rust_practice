fn main() {
    // println!("enter the length of the number of characters:");
    let mut line = String::new();
     let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    // const n: usize =line.trim().parse().expect("Input not an integer");
    
    const N: usize = line.trim().parse().expect("Input not an integer"); 
    let mut ar:[i32;N]=[0;N];
    
    for i in 0..ar.len() {
        let mut line = String::new();
        println!("Enter Here");
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        // println!("{}", line);
        let x: i32 =line.trim().parse().expect("Input not an integer");
        // println!("{}",_x);
        ar[i] = x;
    }
    println!("--------done--------");
    
    for i in 0..ar.len() {
        println!("{}", ar[i]);
    }
    
    println!("--------done--------");

    for val in ar.iter(){
        println!("value is :{}",val);
     }
}