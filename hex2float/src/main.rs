mod hex2float;
use crate::hex2float::hex2float;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let hex_string: &String = &args[1];
    hex2float(hex_string);





//    let test_string: &str = "3ff0 0000 0000 0000";
//    hex2float(test_string);
//    let test_string: &str = "3ff0 0000 0000 0001";
//    hex2float(test_string);
//    let test_string: &str = "3ff0 0000 0000 0002";
//    hex2float(test_string);

//    let bytes = u64::from_str_radix("3ff0000000000001", 16).unwrap();
//    let float = unsafe { std::mem::transmute::<u64,f64>(bytes)  };
//    println!("{}",float);
}
