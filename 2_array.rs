fn main() {
    let arr=[1,2,3,4,5,6,7,8];
    println!("{:?}", arr);// when we want to print compound data type the 
    // in place holder we use  ":?""
    let mut i=0;
    while i<arr.len() {
        println!("{}", arr[i]);
        i+=1;
    }

    let ar:[i32;4]= [1,2,3,10];//here we define the size of array as 4 and data type that we are going to store as integer of 32 bits
    println!("{:?}", ar);
    println!("{:?}", ar.len());
    println!("{}", ar[0]);
     println!("{:?}", ar[1]);

    let mut arr1:[i32;10]= [100;10];//this repeat's the array with 10 of 100's
    println!("{:?}", arr1);

    arr1[6]=12;
    println!("{:?}", arr1);
    println!("{:?}", arr1.len());
    


    
}