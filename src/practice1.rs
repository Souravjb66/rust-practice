fn main(){
    let d1=Drink{
        juice:Flaver::sweet,
        amount:5,
    };
    let d2=Drink{
        juice:Flaver::fruity,
        amount:7
    };
    add(d2);
    
    
}
enum Flaver{
    sweet,
    fruity,
    cold,
}
struct Drink{
    juice:Flaver,
    amount:i32,
}
fn add(drink:Drink){
    match drink.juice{
        Flaver::sweet=>println!("sweet dish"),
        Flaver::fruity=>println!("fruit dish"),
        Flaver::cold=>println!("cold dish"),
        _=>println!("no dish")
    }
    println!("{:?}",drink.amount)

}
