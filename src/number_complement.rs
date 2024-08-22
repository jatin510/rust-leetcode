pub fn find_complement(num: i32) -> i32 {
    let mut temp_num = num;
    let mut result = 0;

    let mut bit_string = String::new();

    while temp_num > 0 {
        let bit = temp_num % 2;
        bit_string.push_str(bit.to_string().as_str());
        temp_num = temp_num / 2;
    }

    let mut i = 0;
    for char in bit_string.chars() {
        if char == '0' {
            // complement logic here
            result += 2i32.pow(i);
        }
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 5;
        assert_eq!(find_complement(num), 2);
    }

    #[test]
    fn test_2() {
        let num = 1;
        assert_eq!(find_complement(num), 0);
    }

    #[test]
    fn test_3() {
        let num = 10;
        assert_eq!(find_complement(num), 5);
    }
}
