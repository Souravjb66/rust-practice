mod House{
    pub mod room1{
        pub fn show(){
            println!("room 1 shown");
            super::room2::show();
            
        }
        
    }
    pub mod room2{
        pub fn show(){
            println!("room 2 shown");
        }
    }
}
fn main(){
    println!("use module");
    room1::show();
    room2::show();
    
}
use crate::House::room1;
use crate::House::room2;