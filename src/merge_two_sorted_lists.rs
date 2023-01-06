use std::any::type_name;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode
{
    pub val: i32,
    pub next: Option< Box<ListNode> >
}

impl ListNode
{
    #[inline]
    pub fn new(val: i32) -> Self
    {
        return ListNode{ val, next: None };
    }

    #[inline]
    pub fn with_next(val: i32, next: ListNode) -> Self
    {
        return ListNode{ val, next: Some(Box::new(next)) };
    }
}

pub fn print_list(list: &Option< Box<ListNode> >)
{
    let iter = &mut list.as_ref();

    let mut result_string = String::new();

    while *iter != None
    {
        if iter.unwrap().next == None
        {
            result_string.push_str(format!("{} -> {}\n", iter.unwrap().val, "None").as_str());
        }
        else
        {
            result_string.push_str(format!("{} -> {}\n", iter.unwrap().val, iter.unwrap().next.as_ref().unwrap().val).as_str());
        }

        *iter = iter.unwrap().next.as_ref();
    }

    result_string.pop();

    println!("{}", result_string);
}

fn print_type_of<T>(_: &T) -> bool
{
    println!("{}", std::any::type_name::<T>());

    return true;
}

pub fn merge_two_lists(list1: Option< Box<ListNode> >, list2: Option< Box<ListNode> >) -> Option< Box<ListNode> >
{
    let mut l1: Option< Box<ListNode> > = list1;
    let mut l2: Option< Box<ListNode> > = list2;

    let merged: Option< Box<ListNode> > = None;

    match (l1, l2)
    {
        (None, None) => return None,
        (None, Some(l2)) => return Some(l2),
        (Some(l1), None) => return Some(l1),
        (Some(_), Some(_)) => {}
    };


    return merged;
}

#[test]
pub fn test()
{
    let list1 = ListNode::with_next(1, ListNode::new(5));
    let list2 = ListNode::new(3);

    let res = ListNode::with_next(1, ListNode::with_next(3, ListNode::new(5)));

    assert_eq!(*merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2))).unwrap(), res);
}
