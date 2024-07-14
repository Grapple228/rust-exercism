pub fn is_armstrong_number(num: u32) -> bool {
    let digits_count: u32 = num.to_string().len() as u32;

    let mut sum: u32 = 0;

    let mut required_sum = num;
    let mut num_to_add;
    let mut digit: u32;
    while required_sum != 0 {
        digit = required_sum % 10;
        num_to_add = digit.pow(digits_count);
        
        match sum.checked_add(num_to_add){
            None => {
                return false;
            },
            Some(ans) =>{
                sum = ans
            }
        }

        required_sum /= 10;
    }

    sum == num
}