use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ptr::NonNull;

impl Solution {
    #[allow(unsafe_code)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut p = &mut head;
        let mut count = 0;
        while p.is_some() {
            p = &mut p.as_mut().unwrap().next;
            count += 1;
        }
        let mut p = &mut head;
        if n == count {
            return head.unwrap().next;
        }
        for i in 0..(count - n - 1) {
            p = &mut p.as_mut().unwrap().next;
        }

        let right_half = p.as_mut().unwrap().next.take().unwrap().next;
        p.as_mut().unwrap().next = right_half;

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end(head, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

}
