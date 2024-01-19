fn main(){
    println!("hello traits");
    let c1=Car{
        engine:"v8".to_owned(),
        no:23,
    };
    let print=Car::show(&c1);
    println!("{:?}",print);
    let b1=Bike{
        engine:"v6".to_owned(),
        model:"ns200".to_owned()
    };
    
    let bike_print:String=Bike::show(&b1);
    println!("{:?}",bike_print);
    Notify(&c1);
    Notify(&b1);
}
struct Car{
    engine:String,
    no:u32
}
struct Bike{
    engine:String,
    model:String
}
trait Speed{
    fn show(&self)->String;
    
}
impl Speed for Car{
    fn show(&self)->String{
        self.engine.to_owned()
    }
}
impl Speed for Bike{
    fn show(&self)->String{
        self.model.to_owned()
    }
}
fn Notify(item:&impl Speed){
    println!("{:?}",item.show());
}