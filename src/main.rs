fn main() {

    let test_mean64 = mean_64(&vec![20, 9_384, 201_923_920_389_782_202, i64::MAX, 239, 923_899]);

    match test_mean64 {
        Some(value) => println!("{}", value),
        None => println!("No value"),
    }
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