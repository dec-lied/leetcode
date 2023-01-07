pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32
{
    let mut seen: Vec<i32> = Vec::with_capacity(nums.len() / 2);

    for num in nums.iter_mut()
    {
        if seen.contains(num)
        {

        }
        else
        {
            seen.push(*num);
        }
    }

    return nums.len() as i32;
}

#[test]
pub fn test()
{
    let mut nums: Vec<i32> = vec![1, 1, 2, 3, 5, 5, 5, 9, 10, 11];
    let nums2: Vec<i32> = vec![1, 2, 3, 5, 9, 10, 11];

    assert_eq!(remove_duplicates(&mut nums), 7);
    assert_eq!(nums, nums2);
}
