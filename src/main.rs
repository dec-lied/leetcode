pub mod merge_two_sorted_lists;

use merge_two_sorted_lists::*;

fn main()
{
    let list1 = ListNode::with_next(1, ListNode::new(5));
    let list2 = ListNode::new(3);

    let res = merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)));

    print_list(&res);
}

