fn main(){
    println!("hello practice 9");
    let give=num::two(122);  //give value in parameter
    match give{
        num::one(a)=>println!("f one"),  //give an argument parameter where the value is catch
        num::two(b)=>add(&b),
        num::three(c)=>println!("f three {}",c)
    }
    
}
fn add(item:&i32){
    println!("hello add :{}",item);
}
enum num{
    one(i32),
    two(i32),
    three(i32)
}