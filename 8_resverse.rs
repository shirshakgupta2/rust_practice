fn main() {
    for i in (0..10).rev() { // .rev() for reversed compatibility iteration
        println!("{}", i);
    }
    let mut a=10;
    while a>=0  {
        println!("{}", a);
        a -= 1;
    }

}