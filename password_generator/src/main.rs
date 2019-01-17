use std::env;
use rand::Rng;

fn main(){

  
  let input:u16 =match env::args().nth(1){
    Some(args1)=> args1 .parse().unwrap(),
    None=>panic!("missing argument"),
  };
  

  let mut rng = rand::thread_rng();
  let mut string:String=String::from("");
  //generate 
  for _i in 0..input {
    let c = (rng.gen_range(32,127) as u8) as char;
    string.push(c);
  }
  println!("{}",string);
}
