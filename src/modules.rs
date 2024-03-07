mod House{
    pub const no:i32=4;
    pub mod bed1{
        pub fn show(){
            println!("bed 1 shown");
        }
    }
    pub mod bed2{
        pub fn show(){
            println!("bed 2 shown");
        }
    }
}
fn main(){
    println!("hello modules");
    House::bed1::show();
    House::bed2::show();
    println!("{}",House::no);


}