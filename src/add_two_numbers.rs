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

pub fn add_two_numbers<T>(
    l1: Option<Box<ListNode<T>>>,
    l2: Option<Box<ListNode<T>>>,
) -> Option<Box<ListNode<T>>> {
    todo!()
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
            )
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
