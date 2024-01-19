fn main(){
   let card = (10,13,14);
   println!("{:?},{:?}",card.0,card.1);
   let (x,y,z)=(2,3,5);
   println!("{x},{y},{z}");
   let (a,b,c)=card;
   println!("{a},{b},{c}");
   println!("{:?}",add());
   let (m,n)=add();
   println!("{m}");

}
fn add()->(i32,i32){
    (1,3)

}