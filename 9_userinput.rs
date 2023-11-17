fn main() {
       let mut line = String::new();
       println!("Enter Here");
       let _b1 = std::io::stdin().read_line(&mut line).unwrap();
       println!("{}", line);
       // for input line where the input is integer
       let _x: i32 =line.trim().parse().expect("Input not an integer");
       println!("{}",_x);
}
