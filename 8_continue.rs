fn main() {
    let mut a=1;
    while a < 10 {
        println!("{}", a);
        a+=1;
        if a==6{
            a+=1;
            continue;
        }
    }
    println!("-----done-----");
    for i in 1..=10{
        if i==6{
          
            continue;// this send the cursor back to top of the loop and increment
            
        }
        println!("{}", i);
    }
}