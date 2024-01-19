fn main(){
    add(&bike::fz);
    let c1=car{
        engine:1200,
        wheel:4
    };
    let c2=car{
        engine:1300,
        wheel:2
    };
    println!("{:?}",c1.engine);
    println!("{:?}",c2.engine);
    
    


}
enum bike{
    duke,
    ns200,
    rc,
    fz
}
struct car{
    engine:i32,
    wheel:i32
}
fn add(b1:&bike){
    match b1{
        bike::duke=>println!("hello duke"),
        bike::ns200=>println!("hello ns"),
        bike::rc=>println!("hello rc"),
        bike::fz=>println!("hello fz")
    } 
}