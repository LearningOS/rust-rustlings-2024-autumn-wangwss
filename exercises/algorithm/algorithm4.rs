/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM DONE
use std::cmp::{self, Ordering};
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
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
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match &mut self.root {
            Some(ref mut root)=>{
                root.insert(value);
            },
            None=>self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut node: Option<&Box<TreeNode<T>>> = self.root.as_ref();
        
        while let Some(item) = node {
            match item.value.cmp(&value) {
                Ordering::Equal => {
                    // key == value
                    return true;
                },
                Ordering::Greater => {
                    // key > value
                    match &item.left {
                        Some(next) => node = Some(&next),
                        None => node = None,
                    }
                },
                Ordering::Less => {
                    // key < value
                    match &item.right {
                        Some(next) => node = Some(&next),
                        None => node = None,
                    }
                }
            }
        };
        false        
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match self.value.cmp(&value) {
            Ordering::Equal=>{
                return;
            },
            Ordering::Greater=>{
                match &mut self.left {
                    Some(ref mut node)=>{
                        node.as_mut().insert(value);
                    },
                    None=>{
                        self.left = Some(Box::new(TreeNode::new(value))) ;
                    }
                }
            },
            Ordering::Less=>{
                match &mut self.right {
                    Some(ref mut node)=>{
                        node.insert(value);
                    },
                    None=>{
                        self.right = Some(Box::new(TreeNode::new(value))) ;
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


