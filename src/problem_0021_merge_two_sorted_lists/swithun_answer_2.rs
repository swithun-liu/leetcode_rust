// use crate::data_structures::ListNode;

// pub struct Solution;

// // ------------------------------------------------------ snip ------------------------------------------------------ //

// use std::mem;

// impl Solution {
//     pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut list1 = list1;
//         let mut list2 = list2;
//         let mut l3 = None;
//         Self::worker(&mut list1, &mut list2, &mut l3);
//         l3
//     }

//     pub fn worker(l1: &mut Option<Box<ListNode>>, l2: &mut Option<Box<ListNode>>, l3:&mut Option<Box<ListNode>>) {
//         match (l1, l2) {
//             (None, None) => { },
//             (None, Some(_)) => { },
//             (Some(_), None) => todo!(),
//             (Some(l1_ref), Some(l2_ref)) => {
//                 if l1_ref.val <= l2_ref.val {
//                     *l3 = *l1;
//                     let next_l1_ref = &mut l3.as_mut().unwrap().next.take();
//                     Self::worker(next_l1_ref, l2, &mut l3.unwrap().next);
//                 } else {
//                 }
//             },
//         }
//     }

// }

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_two_lists(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
