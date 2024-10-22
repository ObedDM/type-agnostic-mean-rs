fn main() {

    let mut channel_stats: (&str, i64, i64) = ("SoleYT", 1_000_000, 1_000_1000);
    channel_stats.1 /= 100_000;

    println!("{}: {}", channel_stats.0, channel_stats.1);



    let _array: [&str; 10] = [""; 10];
    
    let mut error_codes: [u16; 5] = [0; 5];
    error_codes[0] = 105;

    for code in error_codes {
        print!("{} ", code);
    }
    println!("");    

    let rgb: [u8; 3] = [255, 32, 144];

    let rgb_mean = my_function(rgb);

    match rgb_mean {
        Some(value) => println!("the mean of the given rgb values is: {}", value),
        None => println!("No value"),
    }

    let test_mean64 = mean_64(&vec![20, 9_384, 201_923_920_389_782_202, i64::MAX, 239, 923_899]);

    match test_mean64 {
        Some(value) => println!("{}", value),
        None => println!("No value"),
    }

}

fn my_function(array: [u8; 3]) -> Option<u8> {
    println!("Colors: R: {}, G: {}, B: {}", array[0], array[1], array[2]);
    
    let rgb_mean = mean_u8(&array);

    return rgb_mean;
}

fn mean_64(data: &[i64]) -> Option<i64> {

    if data.is_empty() {
        return None; // Returns None if data slice is empty
    }

    let sum: i128 = data.iter().map(|&value| value as i128).sum::<i128>();
    // Iterates through the data slice and transforms each i64 number into i128 to sum to avoid overflow.

    let mean: i64 = (sum / data.len() as i128) as i64;
    //Divides by slice len and returns the mean as i64 to save space
    
    Some(mean)
}

fn mean_u8(data: &[u8]) -> Option<u8> {

    if data.is_empty() {
        return None; // Returns None if data slice is empty
    }

    let sum: u16 = data.iter().map(|&value| value as u16).sum::<u16>();
    // Iterates through the data slice and transforms each i64 number into i128 to sum to avoid overflow.

    let mean: u8 = (sum / data.len() as u16) as u8;
    //Divides by slice len and returns the mean as i64
    
    Some(mean)
}