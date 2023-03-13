use std::fs::File;
use std::io::Read;
use std::error::Error;
fn test() -> Result<(), Box<dyn Error>>{
    let mut str = String::new();
    File::open("helloworld1.rs")?.read_to_string(&mut str)?;
    println!("{str}");
    Ok(())
}

fn main() {
   if let Err(e)=test(){
       panic!("err is {}",e);
   } 
}
