fn main(){
    println!("hello constructor");
    let name:String="nexa".to_owned();
    let n:i32=32;
    let c1=Car::new(&name,n);
    println!("model :{:?} no :{:?}",c1.model,c1.no);
    let c2=Car{
        model:name.clone(),  //clone name here
        no:n
    };
    Car::show(&c1,&name)

    // let call=add(&name,no);
    // println!("model {:?} no {:?}",call.model,call.no);
    
    
}
impl Car{
    fn new(m1:&String,n1:i32)->Car{
        Car{
            model:m1.to_owned(),
            no:n1
        }
    }
    fn show(&self,s1:&String){
        Car{
            model:self.model.to_owned(),
            no:self.no
        };
        println!("success {:?} {:?} {:?}",self.model,self.no,s1);
    }
}
// fn add(m1:&String,n1:i32)->Car{
//     Car{
//         model:m1.to_owned(),
//         no:n1,
//     }

// }


struct Car{
    model:String,
    no:i32
}