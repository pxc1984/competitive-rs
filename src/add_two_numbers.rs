#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode<i32>>>,
    l2: Option<Box<ListNode<i32>>>,
) -> Option<Box<ListNode<i32>>> {
    let mut initial = Some(Box::new(ListNode { val: 0, next: None }));
    let mut current = &mut initial;
    let mut current_l1 = l1.clone();
    let mut current_l2 = l2.clone();

    let mut carry = 0;

    while current_l1.as_ref().is_some() {
        let cur_l1 = current_l1.unwrap();
        let cur_l2 = current_l2.unwrap_or_else(|| Box::new(ListNode { val: 0, next: None }));

        let value = cur_l1.val + cur_l2.val + carry;
        carry = value / 10;
        let value = value % 10;

        let new_node = Some(Box::new(ListNode {
            val: value,
            next: None,
        }));

        if let Some(cur) = current {
            cur.next = new_node;
            current = &mut cur.next;
        }

        current_l1 = cur_l1.next;
        current_l2 = cur_l2.next;
    }

    if carry != 0 {
        let new_node = Some(Box::new(ListNode {
            val: carry,
            next: None,
        }));

        if let Some(cur) = current {
            cur.next = new_node;
            current = &mut cur.next;
        }
    }

    initial.unwrap().next
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
        ];

        for test_case in test_cases {
            assert_eq!(
                add_two_numbers(
                    linked_list_from_vec(test_case[0].clone()),
                    linked_list_from_vec(test_case[1].clone())
                ),
                linked_list_from_vec(test_case[2].clone())
            );
            println!("!!!!!\nTest case {test_case:?} successfull\n!!!!!");
        }
    }

    fn linked_list_from_vec<T: Copy + Default>(arr: Vec<T>) -> Option<Box<ListNode<T>>> {
        if arr.is_empty() {
            return None;
        }

        let mut initial = Some(Box::new(ListNode {
            val: T::default(),
            next: None,
        }));
        let mut current = &mut initial;

        for &value in &arr {
            let new_node = Some(Box::new(ListNode {
                val: value,
                next: None,
            }));

            if let Some(cur) = current {
                cur.next = new_node;
                current = &mut cur.next;
            }
        }

        initial.unwrap().next
    }
}
