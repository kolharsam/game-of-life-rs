pub fn convert_to_u32(num: &str) -> u32 {
    num.to_string().parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_u32_with_positive_number() {
        let converted_num = convert_to_u32("09");
        assert_eq!(converted_num, 9);
    }

    #[test]
    #[should_panic]
    fn test_convert_to_u32_with_negative_number() {
        convert_to_u32("-15");
    }
}
