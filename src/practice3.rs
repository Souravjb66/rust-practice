fn main(){
    let item=Car{
        engine:1200,
        wheel:4
    };
    println!("{:?}",display(&item));//we borrow item or give item address so using &


}
struct Car{
    engine:i32,
    wheel:i32,
}
fn display(car:&Car)->(i32,i32){ //we will recieved a borrow struct so we declare &Car so that we get a borrow method
    
    (car.engine,car.wheel)


}