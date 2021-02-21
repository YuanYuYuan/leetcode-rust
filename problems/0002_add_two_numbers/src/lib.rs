/// NO. 2: Add Two Numbers

pub struct Solution;



pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    vec.iter().rev().fold(None, |head, &v| {
        let mut node = ListNode::new(v);
        node.next = head;
        Some(Box::new(node))
    })
}

pub fn from_vec_forward(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(vec[0])));
    let mut tail = head.as_mut();

    // We need to use take to move out tail in FuMut Closure
    vec.iter().skip(1).for_each(|&v| {
        let t = tail.take().unwrap();
        t.next = Some(Box::new(ListNode::new(v)));
        tail = t.next.as_mut();
    });

    head
}

pub fn from_range(range: i32) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = head.as_mut();
    for i in 1..range {
        let t = tail.take().unwrap();
        t.next = Some(Box::new(ListNode::new(i)));
        tail = t.next.as_mut();
    }
    head
}

pub fn extend(mut l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (&l1, &l2) {
        (None, _) => l2,
        (_, None) => l1,
        _ => {
            let mut tail = l1.as_mut();
            loop {
                let node = tail.unwrap();
                if node.next.is_some() {
                    tail = node.next.as_mut();
                } else {
                    node.next = l2;
                    break;
                }
            }
            l1
        }
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

// ----- submission codes start here -----

#[inline]
pub fn add_val(list: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut carry = val;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut tail = head.as_mut();
    let mut indicator = list.as_ref();

    loop {
        if carry == 0 {
            if let Some(node) = indicator {
                tail.unwrap().next = Some(node.to_owned());
            }
            break;
        } else {
            if let Some(node) = indicator {
                let sum = node.val + carry;
                carry = if sum >= 10 { 1 } else { 0 };
                let tmp = tail.unwrap();
                tmp.next = Some(Box::new(ListNode::new(sum % 10)));
                tail = tmp.next.as_mut();
                indicator = node.next.as_ref();
            } else {
                tail.unwrap().next = Some(Box::new(ListNode::new(carry)));
                break;
            }
        }
    }
    head.unwrap().next
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        // dummy head
        let mut head = Some(Box::new(ListNode::new(0)));

        // tail: Some value -> None
        let mut tail = head.as_mut();

        // Difference between
        //     &l1         : &<Option<Box<ListNode>>
        //     l1.as_ref() : <Option<&Box<ListNode>>
        let mut indicator1 = l1.as_ref();
        let mut indicator2 = l2.as_ref();
        let mut carry = 0;

        loop {
            match (indicator1, indicator2) {
                (Some(node1), Some(node2)) => {
                    // numerical computation
                    let sum = node1.val + node2.val + carry;
                    carry = if sum >= 10 { 1 } else { 0 };

                    // Create a new node and link the tail after the new one
                    let tmp = tail.unwrap();
                    tmp.next = Some(Box::new(ListNode::new(sum % 10)));
                    tail = tmp.next.as_mut();

                    // shift the indicators
                    indicator1 = node1.next.as_ref();
                    indicator2 = node2.next.as_ref();
                },
                (None, Some(node)) | (Some(node), None) => {
                    tail.unwrap().next = add_val(Some(node.to_owned()), carry);
                    break
                },
                (None, None) => {
                    if carry > 0 {
                        tail.unwrap().next = Some(Box::new(ListNode::new(carry)));
                    }
                    break
                },
            }
        }
        head.unwrap().next
    }

}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::add_two_numbers(
                from_vec(vec![2, 4, 3]),
                from_vec(vec![5, 6, 4]),
            ),
            from_vec(vec![7, 0, 8]),
        );
        assert_eq!(
            Solution::add_two_numbers(
                from_vec(vec![0]),
                from_vec(vec![0]),
            ),
            from_vec(vec![0]),
        );
        assert_eq!(
            Solution::add_two_numbers(
                from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
                from_vec(vec![9, 9, 9, 9]),
            ),
            from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]),
        );
    }
}
