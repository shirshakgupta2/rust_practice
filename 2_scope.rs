
    fn main(){
        //variable  that can be used till when
        let age=17;
        println!("{}",age);
        {   // rank will be access only between these brackets 
            let age =20;
            let rank=1;
           println!("{}",rank);
           println!("{}",age);// called as shadowing 
        }
       //cannot accesss rank herebecause its outside
       // the curly bracket  println!("{}",rank);
       
       println!("{}",age);
      
    }
    
