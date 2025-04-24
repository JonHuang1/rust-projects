mod hex2float;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let hex_string = hex2float::build_hex_string(&args);
    hex2float::hex2float(&hex_string);

    // 3ff0 0000 0000 0000
    // 3ff0 0000 0000 0001
    // 3ff0 0000 0000 0002
}
