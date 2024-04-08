/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;
#[derive(Clone)]
#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,T: Clone
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,T: Clone
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,T: Clone
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
    T: Ord,T: Clone
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut back = false;
        match & mut self.root {
            None => {
                self.root = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
            }
            Some(treenode) => {
                let mut node : Option<Box<TreeNode<T>>>;
                let mut sign: bool;
                if treenode.value > value {
                    node = treenode.left.take();
                    sign = true;
                } else {
                    node = treenode.right.take();
                    sign = false;
                }
                let mut pare = self.root.take();
                loop {
                    match & mut node {
                        None => {
                            if sign {
                                pare.unwrap().left = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
                            } else {
                                pare.unwrap().right = Some(Box::<TreeNode<T>>::new(TreeNode::new(value)));
                            }
                            break;
                        }
                        Some(child) => {
                            if sign {
                                pare = pare.unwrap().left;
                            }
                            else{
                                pare = pare.unwrap().right;
                            }
                            if child.value > value {
                                node = child.left.take();
                                sign = true;
                            } else {
                                node = child.right.take();
                                sign = false;
                            }
                            continue;
                        }
                    }
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        true
    }
}

impl<T> TreeNode<T>
where
    T: Ord,T: Clone
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
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
