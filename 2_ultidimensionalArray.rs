fn  main() {
    let a=[[100,101],[102,103],[104,105],[106,107]];
    println!("{:?}", a.len());
    
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            print!("{}  ", a[i][j]);
        }
        println!();    
    }

    
    let mut arr = [0; 5];
    // get input from user
    println!("Enter 5 numbers: ");
    for i in 0..5 {
        arr[i] = read!();
    }
    // print array
    println!("The array is: {:?}", arr);
}