fn main(){
    
    let llb=add(Car::Maruti);
    println!("{:?}",llb);
    
    
}
enum Car{
    Alto,
    Maruti,
    Jagwar,
}
fn add(my:Car){
    match my{
        Car::Alto=>println!("alto car"),
        Car::Maruti=>println!("maruti car"),
        Car::Jagwar=>println!("jagwar car"),
        _=>println!("hhhhhs")
    }
}
