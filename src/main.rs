pub mod traits;
mod modules;

fn main() {
    println!("Hello, function");
    let a =12;
    if a<10{
        println!("small number");
    }else{
        println!("big number");
    }
    let mut b:i32=0;
    loop{
        if b>5{
            break;
        }
        println!("a :{:?}",b);
        b+=1;
    }
    println!("{b}");
    let mut v = 0;
    while v<5 {
        println!("printing no :{:?}",v+2);
        v =v+1;
    }
    
}
