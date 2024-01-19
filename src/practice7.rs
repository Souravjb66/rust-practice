fn main(){
    println!("hello practice 7");
    let my=Numbers{
        one:2,
        two:4,
    };
    Numbers::mon(&my);
    let num:u32=12u8 as u32; //converting u8 to u32 its excute in similar data type
    println!("{}",num);
    

    
}
struct Numbers{
    one:i32,
    two:i32,
}
impl Numbers{  // impl is like a object its first implement class then it define function which is use for that class impl name is same as struct name which we use to define function for
    fn mon(item:&Numbers){
        println!("{:?}",item.two);
    }
}