fn main(){
    println!("hello");
    let mut s1:String=String::from("sourav");
    let mut c:String=String::from("borah");
    let mut s2=& mut s1;
    println!("{s2}");
    *s2=String::from("kol"); //it modify value in address location like c++ so it change original value. every time doing things with s2 we must use *s2 
    println!("{s2}");
    s2=& mut c;
    println!("{}",s2);
    println!("{s1}");
    /////////////////
    let pop:String=String::from("sumitra");
    let lol:String=String::from("hazo");
    let mut ug=& pop;//if first assign value is not mute or not give permission of mute then any value whcih is mute or unmute can assign ....
    println!("ug :{ug}");
    println!("pop :{pop}");
    ug=&lol;
    println!("ug :{ug}");
    println!("pop :{pop}");
    /////
    let mut m1:String=String::from("heyyy");
    let mut m2:String=String::from("kooooo");
    let mut ml=& mut m1;//we cant assign a unmute value to a variable which is previoulsy assign muted value...
    println!("{ml}");
    ml=& mut m2;
    

    
}