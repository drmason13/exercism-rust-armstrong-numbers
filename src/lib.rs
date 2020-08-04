pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = digits(num);

    let mut sum = 0;
    let mut temp = num;
    while temp > 0 {
        let digit = temp % 10;
        temp /= 10;
        sum += digit.pow(num_digits);
    }

    sum == num
}

pub fn digits(num: u32) -> u32 {
    (num as f32).log10() as u32 + 1
}
