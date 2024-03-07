fn main(){
    println!("hello practice 10");
    let c1=Car{
        name:"honda suffer".to_owned(),
        top:120
    };
    println!("{:?}",Car::work(&c1.name));
    println!("{:?}",Car::time(c1.top));

}
trait Speed{
    fn work(name:&String)->String;
    fn time(no:u32)->u32;

}
struct Car{
    name:String,
    top:u32,
    

}
impl Speed for Car{
    fn work(name:&String)->String{
        return name.to_owned();
    }
    fn time(no:u32)->u32{
        return no+12;
    }
}