fn main(){
    println!("arrays");
    let arr=[1,3,4];
    println!("{:?}",arr.len());
    let pr:[i32;4]=[1,2,3,4];
    println!("{:?}",pr.len());
    let num:&mut[i32]=&mut [1,2,3,45]; // we cant create array without giving size and type , but we can give refference of an array of a type
    println!("{:?}",num[1]);
    num[0]=8;
    println!("{:?}",num);
    let v1:Vec<String>=vec!["a".to_owned(),"b".to_owned()];
    println!("{:?}",v1[1]);
    let v2:Vec<u32>=vec![1,2,3,12];
    println!("{:?}",v2.len());
    let my1=num{
        one:11,
        two:22
    };
    let my2=num{
        one:33,
        two:44,
    };
    let v3:Vec<num>=vec![
        my1,
        my2

    ];
    for i in v3{
        println!("{:?},{:?}",i.one,i.two); //think like list now first i have first object m1 so i=my , we can use i.one like to get the value
    }
    for m in num{
        println!("{:?}",m);
    }
    let mut vect:Vec<i32>=Vec::new();
    
    vect.push(1);
    vect.push(2);
    vect.push(7);
    
    println!("{:?}",vect[1]);

    let mut bol:Vec<i32>=vec![];
    bol.push(12);
    bol.push(13);

    println!("{:?}",bol);


}

struct num{  //we can define it inside a function but then we can access it inside that function so we define it outside
    one:i32,
    two:i32,
}

