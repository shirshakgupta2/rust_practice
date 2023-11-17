fn main(){
    let mut line = String::new();
    println!("Enter Here");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();

    let x: i32 =line.trim().parse().expect("Input not an integer");
   let mut count=1;
    for i in 1..=x{
        //print!("{} ",i);
        if i%2!=0{
            for j in 1..=x{
                if count<10{
                    print!("0{} ",count);
                    if(count<5){
                        count+=1;
                    }
                    
                }
                else{
                    print!("{} ",count);
                    count+=1;
                }
    
            }
            count+=x;
            
        
        }
        else{
            for j in 1..=x{
                if count<10{
                    print!("0{} ",count);
                    count-=1;
                }
                else{
                    print!("{} ",count);
                    count-=1;
                }
            }
            count+=x+1;
            ;
            
        }
        
        println!("count: {}",count);

        println!();
    }
}
