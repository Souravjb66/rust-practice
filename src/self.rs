fn main(){
    println!("hello self");
    let l1=Car{
        model:10,
        no:22
    };
    println!("{:?}",l1.total());  //we can do this if parameter only have self
    println!("{:?}",Car::total(&l1)); // this is also use for self

    /////////
    let a:i32=4i32;
    Car::secend(&l1,a); //if paramenter have self and other argument
    l1.secend(a);// also we can use this




}

struct Car{
    model:i32,
    no:i32
}
impl Car{
    fn total(&self)->i32{
        self.model+self.no
    }
    fn secend(&self,item:i32){
        println!("{:?},{:?}",self.model,item)
    }
}