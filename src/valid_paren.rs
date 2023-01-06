pub fn is_valid(s: String) -> bool
{
    if s.len() % 2 != 0
    {
        return false;
    }

    use std::collections::HashMap;

    let bracket_map: HashMap<&str, &str> = HashMap::from
    (
        [ ("(", ")"), ("[", "]"), ("{", "}") ]
    );

    let mut brackets = s.clone();

    let mut l: usize = 0;
    let mut r: usize = 1;

    while brackets.len() != 0 && r != brackets.len()
    {
        if let Some(bracket) = bracket_map.get(&brackets[l..r])
        {
            if bracket.to_owned() == &brackets[l + 1..r + 1]
            {
                brackets.remove(l);
                brackets.remove(l);

                l = 0;
                r = 1;
            }
            else
            {
                l += 1;
                r += 1;
            }
        }
        else
        {
            l += 1;
            r += 1;
        }

        println!("brackets: {}. l: {}, r: {}", brackets, l, r);
    }

    if brackets.len() == 0
    {
        return true;
    }
    else
    {
        return false
    }
}

#[test]
pub fn test()
{
    assert_eq!(is_valid("()[]{}".to_string()), true);
    assert_eq!(is_valid("([{()}[]])[[]]{()}".to_string()), true);
    assert_eq!(is_valid("".to_string()), true);
    assert_eq!(is_valid("}}()".to_string()), false);
}
