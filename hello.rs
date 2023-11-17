fn main(){
    println!("Hello world");
    println!("123");
    let name="shir_shak";
    println!("{}",name);
    let na_me1="shirshak";
    println!("{}",na_me1);
    println!("{}",1+2);
    println!("{}",1-2);
    println!("{}",1*2);
    println!("{}",1/2);
    println!("{}",1.0/2.0);
    println!("{}",11%2);
    let age=17;
    println!("{}",age);

    another_function();

    another_function1(5);
}


fn another_function1(x: i32) {
    println!("The value of x is: {}",x);
}
fn another_function() {
    println!("Another function.");
}

