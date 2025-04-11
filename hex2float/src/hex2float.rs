pub fn hex2float(hex: &str) {
    println!("input: {}", hex);
    let bin_vec: Vec<u8> = hex2binary(hex); 
    binary2float(&bin_vec);
    println!("");
}

fn binary2float(bin_vec: &Vec<u8>) {
    let sign:f64;
    if let 1 = bin_vec[0] { sign = -1.0;}
    else { sign = 1.0; }
    let exp:&[u8] = &bin_vec[1..12];
    let mantissa:&[u8] = &bin_vec[13..64];
    //println!("exp: {:?}", exp);
    let exp_dec: i64 = binary2decimal(exp);
    //let mantissa_dec: f64 = mantissa2decimal(mantissa);
    let mantissa_dec: i64 = binary2decimal(mantissa);
    let base: f64 = 2.0;
    let result: f64;
    if let 0 = exp_dec {
        result = sign * base.powf(-1022.0) * (base.powf(mantissa_dec as f64));
    }
    else {
        result = sign * base.powf((exp_dec - 1023) as f64) * (1.0 + base.powf(mantissa_dec as f64 - 53.0));
    }
    println!("{}", result);
}

fn mantissa2decimal(bin_array: &[u8]) -> f64 {
    let mut result: f64 = 0.0;
    let base: f64 = 2.0;
    for (i, bit) in bin_array.iter().enumerate() {
        result = result + *bit as f64 / base.powf(i as f64);
    }
    return result;
}

fn binary2decimal(bin_array: &[u8]) -> i64 {
    let mut result: i64 = 0;
    for bit in bin_array{
        result = result * 2 + *bit as i64;
    }
    return result;
}

fn hex2binary(hex: &str) -> Vec<u8> {
    let mut bin_vec: Vec<u8> = Vec::new();
    let hex_string = hex.to_lowercase();
    for letter in hex_string.chars() {
        match letter {
            '0' => {bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(0)},

            '1' => {bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(1)},

            '2' => {bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(0)},

            '3' => {bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(1)},

            '4' => {bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(0)},

            '5' => {bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(1)},

            '6' => {bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(0)},

            '7' => {bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(1)},

            '8' => {bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(0)},
                    
            '9' => {bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(0);
                    bin_vec.push(1)},

            'a' => {bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(0)},

            'b' => {bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(1);
                    bin_vec.push(1)},

            'c' => {bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(0)},

            'd' => {bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(0);
                    bin_vec.push(1)},

            'e' => {bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(0)},

            'f' => {bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(1);
                    bin_vec.push(1)},
            ' ' => continue,
            _ => panic!("non hex value in input: {letter}"),
        }
    }

    bin_vec.resize(64,0);

    return bin_vec;
}
