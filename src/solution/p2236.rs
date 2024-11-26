// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let root_node = root.borrow();
            let left_child = root_node.left.as_ref().unwrap().borrow();
            let right_child = root_node.right.as_ref().unwrap().borrow();
            return root_node.val == left_child.val + right_child.val ;
        }
        false
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
      Solution::check_tree(None);
    }

}