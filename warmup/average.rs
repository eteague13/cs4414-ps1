use std::{os, float};

fn main() {
  let args: ~[~str] = os::args();
  let mut count = 1; 
  let mut sum = 0.0;
  let mut numNumbers = 0;
  while count < args.len() {
	match float::from_str(args[count]) {
		Some(num) => {sum+=num; numNumbers+=1}
		None => {println(fmt!("Bad input: %s", args[count]))}
	};
   	count+=1;
	
  }
  println(fmt!("Average: %?", sum as float / (numNumbers as float)));

}
