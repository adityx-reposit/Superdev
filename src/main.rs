
#[derive(Debug)]
struct User{
    name:String,
    age:u32
}

impl std::fmt::Display for User{
    fn fmt (&self , f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f,"your name is {}, age is {}",self.name,self.age)
    }
}


fn main(){
    let u = User{
        name:String::from("Aditya"),
        age:21
    };


    println!("{:?}",u);
}