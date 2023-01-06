pub fn roman_to_int(s: String) -> i32
{
    use std::collections::HashMap;

    let mut value: i32 = 0;
    let numeral_value: HashMap<String, i32> = HashMap::from(
    [
        ("I".to_string(), 1),
        ("V".to_string(), 5),
        ("X".to_string(), 10),
        ("L".to_string(), 50),
        ("C".to_string(), 100),
        ("D".to_string(), 500),
        ("M".to_string(), 1000)
    ]);

    let mut curr: usize = 0;

    while curr < s.len()
    {
        // final char
        if curr == s.len() - 1
        {
            value += numeral_value.get(&s[curr..curr + 1]).unwrap();
        }
        // not final char
        else
        {
            if numeral_value.get(&s[curr..curr + 1]) < numeral_value.get(&s[curr + 1..curr + 2])
            {
                value += numeral_value.get(&s[curr + 1..curr + 2]).unwrap() - numeral_value.get(&s[curr..curr + 1]).unwrap();
                curr += 1;
            }
            else
            {
                value += numeral_value.get(&s[curr..curr + 1]).unwrap();
            }
        }

        curr += 1;
    }

    return value;
}

#[cfg(test)]
mod tests
{
    use super::roman_to_int;

    #[test]
    fn roman_tests()
    {
        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("MCMXIV")), 1914);
    }
}
