fn main(){
    println!("hiii");
    add(3,"sourav".to_owned());  //we cant send single string becase by default its borrow string
    let n1:i32=10;
    let n2:i32=5;
    num(n1,n2);
    println!("{:?}",num(n1,n2));

    // let p1:i32=5;
    // let p2:i32=p1;
    // println!("{p2}");
    // println!("{p1}");
    println!("{:?}",rec(1));

}
fn add(a:i32,b:String){
    println!("heyyy my name :{:?} roll no:{:?}",b,a);
    
    
    println!("uppercase name:{:?}",b.to_uppercase()); //to convert to uppercase
    
}
fn num(a:i32,b:i32)->i32{
    (a+b)*2
}

fn rec(i:i32)->i32{
    let v:i32=1;
    if i==5 {
        println!("print");
        i
    }
    else{
        println!("{i}");
        
        
        rec(i+1)+2
        
        
    }
    

}