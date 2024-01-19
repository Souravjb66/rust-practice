fn main(){
    println!("hello option");
    let b1=Car{
        model:Some(12),
        no:None,
    };
    println!("{:?}",b1.no);
    let first=my::two;
    match first{
        my::one=>println!("hello one"),
        my::two=>println!("hello two"),
        my::three=>println!("hello three"),
        _=>println!("nothing")
    }
}
struct Car{
    model:Option<i32>,
    no:Option<i32>,
}
enum my{
    one,
    two,
    three
}

