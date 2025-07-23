


// #[derive(Debug)]
// struct User{
//     name:String,
//     age:u32
// }



// fn main() {
//     let u = User{
//         name:String::from("Aditya"),
//         age:21
//     };


//     println!("{:?}",u);
// }

// procedural macros 

use std::fmt::Error;



trait Serialize{
    fn serialize(&self)->Vec<u8>;
}

trait Desirialize:Sized{
    fn desirialize(v:Vec<u8>)-> Swap;
}



struct Swap{
    qty_1:u32,
    qty_2:u32,
    qty_3:u32,
}

impl Serialize for Swap{
    fn serialize(&self)->Vec<u8> {
       let mut v = Vec::new();
            v.extend(self.qty_1.to_be_bytes());
            v.extend(self.qty_2.to_be_bytes());
            v.extend(self.qty_3.to_be_bytes());
          
       return v;
        
    }
}


impl Desirialize for Swap {
    fn desirialize(v:Vec<u8>)-> Swap {
        let qty1=v[0..3];
        let qty2=v[0..4];
        let qty3=v[0..6];

        
        return Swap{
            qty_1,
            qty_2,
            qty_3
        }
    }
}


fn main(){
   let swap= Swap{
       qty_1:5,
       qty_2:7,
       qty_3:3,
   };

   let function = swap.serialize();
   println!("{:?}",function)
    
  
}



