use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fmt::Result as Result;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct BST<K, V> {
    pub root: Link<K, V>,
}

#[derive(Debug)]
pub struct Node<K, V> {
    pub left: Link<K, V>,
    pub right: Link<K, V>,
    pub key: K,
    pub value: V,
}

impl<K: Clone + Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            key,
            value,
        }
    }
}


impl<K: Ord + PartialEq, V> BST<K, V> {
    pub fn insert(&mut self, input: Node<K, V>) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            if input.key >= node.key {
                current = &mut node.right;
            } else {
                current = &mut node.left;
            } 
        }
        *current = Some(Box::new(input));
    }

    pub fn height(&self) -> usize {
        let mut queue = VecDeque::new();
        let mut height = 0;
        if self.root.is_none() {
            return height;
        } else {
            queue.push_back(self.root.as_ref());
            height += 1;
            while let Some(node) = queue.pop_front() {
                let current = node.unwrap();
                height += 1;
                if current.left.is_some() && current.right.is_some() {
                    queue.push_back(current.left.as_ref());
                    queue.push_back(current.right.as_ref());
                } else if current.left.is_some() {
                    queue.push_back(current.left.as_ref());
                } else if current.right.is_some() {
                    queue.push_back(current.right.as_ref());
                } else {
                    break;
                }
            }
            return height;
        }
    }

    pub fn inorder<'a>(&'a self) -> InOrderIter<'a, K, V> {
        InOrderIter {
            stack: Vec::new(),
            current: self.root.as_ref(),
        }
    }
    pub fn preorder<'a>(&'a self) -> PreOrderIter<'a, K, V> {
        PreOrderIter {
            stack: Vec::new(),
            current: self.root.as_ref(),
        }
    }

    pub fn postorder<'a>(&'a self) -> PostOrderIter<'a, K, V> {
        PostOrderIter {
            stack: Vec::new(),
            current: self.root.as_ref(),
        }
    }
}


pub struct InOrderIter<'a, K: Ord, V> {
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
    current: Option<&'a Box<Node<K, V>>>,
}
pub struct PreOrderIter<'a, K: Ord, V> {
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
    current: Option<&'a Box<Node<K, V>>>,
}
pub struct PostOrderIter<'a, K: Ord, V>{
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
    current: Option<&'a Box<Node<K, V>>>,
}

impl<'a, K: Ord, V> Iterator for InOrderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.current {
                self.stack.push(self.current);
                self.current = current.left.as_ref();
            } else {
                if let Some(node) = self.stack.pop() {
                    let current = node.unwrap();
                    let result = (&current.key, &current.value);
                    self.current = current.right.as_ref();
                    return Some(result);
                } else {
                    return None;
                }
            }
        }

    }
}

impl<'a, K: Ord, V> Iterator for PreOrderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.current {
                self.stack.push(self.current);
                let result = (&current.key, &current.value);
                self.current = current.left.as_ref();
                return Some(result);
            } else {
                if let Some(node) = self.stack.pop() {
                    let current = node.unwrap();
                    self.current = current.right.as_ref();
                    if self.current.is_some() {
                        self.stack.push(self.current);
                    }
                } else {
                    return None;
                }
            }
        }
    }
}
/*impl<K: Ord, V> Display for BST<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        todo!("Displays a tree")
    }
}*/

impl<'a, K: Ord, V> Iterator for PostOrderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while let Some(current) = self.current {
                if current.right.is_some() {
                    self.stack.push(current.right.as_ref());
                }
                self.stack.push(self.current);
                self.current = current.left.as_ref();
            } if self.stack.is_empty() {
                return None;
            } 
            if let Some(node) = self.stack.pop() {
                let current = node.unwrap();
                if !self.stack.is_empty() && current.right.is_some() &&
                    self.stack.get(self.stack.len()-1)
                        .unwrap().unwrap().key == current.right.as_ref().unwrap().key {
                            self.stack.pop();
                            self.stack.push(Some(current));

                            self.current = current.right.as_ref();
                } else {
                    let result = (&current.key, &current.value);
                    self.current = None;
                    return Some(result);
                }
            } 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let node = Node::new(5, "hello");
        let tree = BST { root: Some(Box::new(node)), };
        assert!(tree.root.is_some())
    }
    #[test]
    fn insert() {
        let node = Node::new(5, "hello");
        let node2 = Node::new(11, "world");
        let mut tree = BST { root: Some(Box::new(node)), };
        tree.insert(node2);
        assert!(tree.root.unwrap().right.is_some());
    }
    #[test]
    fn absence() {
        let node = Node::new(5, "hello");
        let node2 = Node::new(11, "world");
        let mut tree = BST { root: Some(Box::new(node)), };
        tree.insert(node2);
        assert!(tree.root.unwrap().left.is_none());
    }
    #[test]
    fn height() {
        let node = Node::new(5, "hello");
        let node2 = Node::new(11, "world");
        let node3 = Node::new(20, "rust");
        let node4 = Node::new(3, "crate");
        let node5 = Node::new(9, "mod");
        let node6 = Node::new(7, "borrow");
        let node7 = Node::new(14, "checker");
        let mut tree = BST { root: Some(Box::new(node)), };
        tree.insert(node2);
        tree.insert(node3);
        tree.insert(node4);
        tree.insert(node5);
        tree.insert(node6);
        tree.insert(node7);
        assert_eq!(tree.height(), 3);
    }
}
