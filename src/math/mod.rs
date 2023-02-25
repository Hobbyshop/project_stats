pub fn get_sum(nums: &Vec<u64>) -> u64 {
    let mut i: u64 = 0;

    for num in nums {
        i += num;
    }

    i
}

pub fn get_percent(full: &u64, part: &u64) -> f32 {
    let quotient = *part as f64 / *full as f64;
    quotient as f32
}
