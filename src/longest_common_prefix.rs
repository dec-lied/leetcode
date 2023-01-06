fn is_shared_prefix(strs: &Vec<String>, prefix: &str) -> bool
{
    for str in strs.iter()
    {
        if &str[0..prefix.len()] != prefix
        {
            return false;
        }
        else
        {
            println!("{} is not {}", &str[0..prefix.len()], prefix);
        }
    }

    return true;
}

pub fn longest_common_prefix(strs: Vec<String>) -> String
{
    let mut shortest_len: usize = strs[0].len();

    for i in 1..strs.len()
    {
        if strs[i].len() < shortest_len
        {
            shortest_len = strs[i].len();
        }
    }

    let mut prefix: String = String::new();

    let mut r: usize = 1;

    while r < shortest_len && is_shared_prefix(&strs, strs[0].get(0..r).unwrap())
    {
        prefix = strs[0][0..r].to_owned();

        r += 1;
    }

    return prefix;
}

#[test]
pub fn test()
{
    assert_eq!
    (
        longest_common_prefix
        (
            vec!["a"]
                .into_iter()
                .map(String::from)
                .collect()
        ),
        "a".to_string()
    );
}
