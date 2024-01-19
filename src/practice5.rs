fn main(){
    let a:i32=5;
    {
        let b:i32=6;
        println!("{b}");
    }
    println!("{a}");
    let a:i32=10;  //re creating the variable its called shadowing
    println!("{a}");
    let arr:&str="heyy go";
    let bb=arr;
    println!("{bb}");
    println!("{arr}");
    
}