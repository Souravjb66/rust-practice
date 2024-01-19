fn main(){
    println!("hello practice");
    let car=Car{
        engine:"bs6".to_owned(),
        wheel:4,
        brand:"alto800".to_owned(),
    };
    Car::show(&car);
    println!("{:?}",Car::give(&car));
    Numbers::num(&Numbers::two);  //passing reference of two and it use Numbers::two not like struct n.brand because enum not use instance
    
}
struct Car{
    engine:String,
    wheel:i32,
    brand:String,

}
enum Numbers{
    one,
    two,
    three,
}
impl Car{
    fn show(item:&Car){   //taking a borrow struct name car
        println!("engine :{:?} wheel {:?} brand {:?}",item.engine,item.wheel,item.wheel);
    }
    fn give(one:&Car)->String{ //taking a borrow struct
        let a:&String=&one.brand; //store brand value in a by referece we give both side & because in a side we declare that we want refference not actual value and in one.brand side we declare that we give refference
        if a=="alto800" {
            return a.to_owned();
        }
        return "not found".to_owned();

    }
}
impl Numbers{
    fn num(n:&Numbers){
        match n{
            Numbers::one=>println!("printing one"),
            Numbers::two=>println!("printing two"),
            Numbers::three=>println!("printing three"),
            _=>println!("wron info given")
        }
    }
}