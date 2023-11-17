fn main() {
    let mut line = String::new();
    println!("Enter Here");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    let x: i32 =line.trim().parse().expect("Input not an integer");
    
    for i in (1..=x).rev() {
        for j in (1..=x).rev(){
            if j == i ||j==(x-i+1){
               if i<=x/2{
                  if j==i{
                    print!("{}",x-i+1);
                  }
                  else{
                    print!("{}",i);
                  }
            }
            else{
                print!("{}",j);
            }
        }
        else{
                print!(" ");
            }
        }
        println!();
    }
        
}

