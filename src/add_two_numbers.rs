use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedList<T>(pub Option<Box<ListNode<T>>>);

impl<T: std::fmt::Debug + Clone> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<T> = self.into();
        write!(f, "{:?}", v)
    }
}

impl<T> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut iter = vec.into_iter();
        let first = match iter.next() {
            Some(v) => v,
            None => return LinkedList(None),
        };

        let mut head = Box::new(ListNode::new(first));
        let mut tail = &mut head;

        for value in iter {
            tail.next = Some(Box::new(ListNode::new(value)));
            tail = tail.next.as_mut().unwrap();
        }

        LinkedList(Some(head))
    }
}

impl<T: Clone> From<&LinkedList<T>> for Vec<T> {
    fn from(list: &LinkedList<T>) -> Self {
        let mut vec = Vec::new();
        let mut current = list.0.as_deref();

        while let Some(node) = current {
            vec.push(node.val.clone());
            current = node.next.as_deref();
        }

        vec
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    add_two_numbers_linked(LinkedList(l1), LinkedList(l2)).0
}

pub fn add_two_numbers_linked(l1: LinkedList<i32>, l2: LinkedList<i32>) -> LinkedList<i32> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    let mut carry = 0;

    // мы не можем менять сигнатуру функции, так что вот так
    let mut l1 = l1.0.clone();
    let mut l2 = l2.0.clone();

    while l1.is_some() || l2.is_some() || carry != 0 {
        let v1 = l1.as_ref().map_or(0, |n| n.val);
        let v2 = l2.as_ref().map_or(0, |n| n.val);

        let sum = v1 + v2 + carry;
        carry = sum / 10;

        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();

        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);
    }

    LinkedList(Some(dummy.next.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let test_cases = [
            [vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]],
            [vec![0], vec![0], vec![0]],
            [
                vec![9, 9, 9, 9, 9, 9, 9],
                vec![9, 9, 9, 9],
                vec![8, 9, 9, 9, 0, 0, 0, 1],
            ],
            [vec![2, 4, 9], vec![5, 6, 4, 9], vec![7, 0, 4, 0, 1]],
        ];

        for test_case in test_cases {
            let res =
                add_two_numbers_linked(test_case[0].clone().into(), test_case[1].clone().into());
            let expected: LinkedList<i32> = test_case[2].clone().into();
            assert_eq!(res, expected, "expected {expected} but got {res}");
            println!("pass {test_case:?}");
        }
    }
}
