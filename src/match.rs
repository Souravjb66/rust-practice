fn main(){
    let cal=true;
    match cal{  //bollean have only 2 value true and false so we cant assign a condition for other value 
        true=>println!("true click"),
        false=>println!("false clicked"),
        
    }
    let click:i32=2;
    match click{
        1=>println!("1 click"),
        2=>println!("2 click"),
        _ =>println!("nothing"),
    }
    //////////
    
   
}