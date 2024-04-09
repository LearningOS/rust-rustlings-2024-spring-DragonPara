/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
#[derive(Clone, Debug)]
struct TreeNode<T>
where
    T: Ord,
    T: Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
    T: Clone,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
    T: Clone,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
    T: Clone,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => {
                self.root = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
            }
            Some(treenode) => {
                // treenode.insert(value);
                let mut fuck = treenode; // treenode不能切换引用的对象，声明fuck做这件事
                loop {
                    if fuck.value == value {
                        return;
                    } else if fuck.value > value {
                        if fuck.left.is_none() {
                            fuck.left = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
                            return;
                        } else {
                            if let Some(ref mut node) = fuck.left {
                                fuck = node;
                            }
                        }
                    } else {
                        match fuck.right {
                            None => {
                                fuck.right =
                                    Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
                                return;
                            }
                            Some(ref mut node) => {
                                fuck = node; //因为这行代码，fuck.right 不能和前面的 
                                             //self.root 一样使用&mut 而是单独在 Some 中 ref mut
                            }
                        }
                    }
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
            None => {
                return false;
            }
            Some(node) => {
                return node.search(value);
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
    T: Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if self.value == value {
            return;
        } else if self.value > value {
            match &mut self.left {
                None => {
                    self.left = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
                }
                Some(lnode) => {
                    lnode.insert(value);
                }
            }
        } else {
            match &mut self.right {
                None => {
                    self.right = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
                }
                Some(rnode) => {
                    rnode.insert(value);
                }
            }
        }
        //TODO
    }
    fn search(&self, value: T) -> bool {
        if self.value == value {
            return true;
        } else {
            if self.value > value {
                match &self.left {
                    None => {
                        return false;
                    }
                    Some(lnode) => {
                        return lnode.search(value);
                    }
                }
            } else {
                match &self.right {
                    None => {
                        return false;
                    }
                    Some(rnode) => {
                        return rnode.search(value);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
