fn main(){
    println!("hello");
    let mut a:i32=12;
    let mut c:i32=10;
    println!("{}",a); //12
    let mut b:&mut i32=& mut a; //b accept a reference of mutable variable assigning mutable a
    println!("{}",b); //12
    b=&mut c; //assign c to b
    println!("{}",b); //10
    *b=4i32; //dereference variable c which address store in b and change c value to 4 then saved address of c in b
    println!("{}",b); //4
    println!("{}",a); //12
    println!("{}",c); //4
    


}