// creating custom declaritive macro 


use std::fmt::Error;

fn main (){

   let swap1=Swap{
    qty1:1,
    qty2:2
   };

    let v= [0,0,0,1,0,0,0,2];

   println!("{:?}",swap1.serialize()) ;
   println!("{:?}",Swap::deserialize(v.to_vec()));
}

#[derive(Debug)]
struct Swap{
    qty1:u32,
    qty2:u32
}


trait Serialize {
    fn serialize(&self)->Vec<u8>;
}
trait Deserialize{
    // The error is: "the size for values of type `Self` cannot be known at compilation time"
    // This is because `Self` might not be `Sized`. We need to bound Self: Sized.
    fn deserialize(v:Vec<u8>)-> Result<Self,Error> where Self: Sized;
}




impl Serialize for Swap{
 fn serialize(&self)->Vec<u8> {
     let mut v=Vec::new();
     v.extend_from_slice(&self.qty1.to_be_bytes());
     v.extend_from_slice(&self.qty2.to_be_bytes());
     v
 }

}



impl Deserialize for Swap{

    fn deserialize(v:Vec<u8>)-> Result<Self,Error> {


        if v.len()< 8 {
            return Err(Error);
        }
      // Convert slices to fixed-size arrays for from_be_bytes
      let qty1 = u32::from_be_bytes([v[0], v[1], v[2], v[3]]);
      let qty2 = u32::from_be_bytes([v[4], v[5], v[6], v[7]]);
    
        return Ok( Self {
           qty1,
           qty2
        });


    }
}


