use std::collections::VecDeque;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct BST<K, V> {
    pub root: Link<K, V>,
}

#[derive(Debug)]
pub struct Node<K, V> {
    left: Link<K, V>,
    right: Link<K, V>,
    key: K,
    value: V,
}

impl<K: Ord, V: PartialEq> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            key,
            value,
        }
    }
}

impl<K: Ord, V: PartialEq> BST<K, V> {
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

    pub fn inorder<'a>(&'a self) -> InOrderIter<'a, K, V> {
        InOrderIter {
            stack: Vec::new(),
            current: self.root.as_ref(),
        }
    }
    pub fn preorder<'a>(&'a self) -> PreOrderIter<'a, K, V> {
        PreOrderIter {
            stack: vec![self.root.as_ref()],
        }
    }

    pub fn postorder<'a>(&'a self) -> PostOrderIter<'a, K, V> {
        PostOrderIter {
            stack: Vec::new(),
            current: self.root.as_ref(),
        }
    }

    pub fn values_mut<'a>(&'a mut self) -> ValuesMut<'a, K, V> {
        ValuesMut {
            queue: {
                let mut q = VecDeque::new();
                q.push_back(self.root.as_mut());
                q
            }
        }
    }

    pub fn contains_val(&self, value: &V) -> bool {
        let mut iter = self.preorder();
        while let Some(node) = iter.next() {
            if node.1 == value {
                return true;
            }
        }
        false
    }
}

pub struct IntoIter<K, V> {
    stack: Vec<Option<Box<Node<K, V>>>>,
}

impl<K: Ord, V: PartialEq> IntoIterator for BST<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { 
            stack: vec![self.root], 
        }
    }
}

impl<K: Ord, V: PartialEq> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let node = node.unwrap();
            let result = (node.key, node.value);
            if node.right.is_some() {
                self.stack.push(node.right);
            }

            if node.left.is_some() {
                self.stack.push(node.left);
            }
            Some(result)
        } else {
            None
        }
    }
}

pub struct InOrderIter<'a, K, V> {
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
    current: Option<&'a Box<Node<K, V>>>,
}

impl<'a, K: Ord, V: PartialEq> Iterator for InOrderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.current {
                self.stack.push(self.current);
                self.current = current.left.as_ref();
            } else { // traversed all of the left children from root 
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

pub struct PreOrderIter<'a, K, V> {
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
}

impl<'a, K: Ord, V: PartialEq> Iterator for PreOrderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let node = node.unwrap();
            let result = (&node.key, &node.value);
            if node.right.is_some() {
                self.stack.push(node.right.as_ref());
            }

            if node.left.is_some() {
                self.stack.push(node.left.as_ref());
            }
            Some(result)
        } else {
            None
        }
    }
}

pub struct PostOrderIter<'a, K, V>{
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
    current: Option<&'a Box<Node<K, V>>>,
}

impl<'a, K: Ord, V: PartialEq> Iterator for PostOrderIter<'a, K, V> {
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

pub struct ValuesMut<'a, K, V> {
    queue: VecDeque<Option<&'a mut Box<Node<K, V>>>>,
}

impl<'a, K: Ord, V: PartialEq> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop_front() {
            let node = node.unwrap();
            if let Some(left) = &mut node.left {
                self.queue.push_back(Some(left));
            }
            if let Some(right) = &mut node.right {
                self.queue.push_back(Some(right));
            }
            return Some(&mut node.value);
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tree_setup() -> BST<i32, &'static str> {
        let node = Node::new(4, "hello");
        let node2 = Node::new(2, "world");
        let node3 = Node::new(1, "rust");
        let node4 = Node::new(3, "crate");
        let node5 = Node::new(6, "mod");
        let node6 = Node::new(5, "iter");
        let node7 = Node::new(7, "tree");

        let mut tree = BST { root: Some(Box::new(node)), };

        tree.insert(node2);
        tree.insert(node3);
        tree.insert(node4);
        tree.insert(node5);
        tree.insert(node6);
        tree.insert(node7);

        tree
    }
    #[test]
    fn inorder() {
        let tree = tree_setup();

        assert!(tree.root.is_some());

        let mut iter = tree.inorder();

        assert_eq!(iter.next(), Some((&1, &"rust")));
        assert_eq!(iter.next(), Some((&2, &"world")));
        assert_eq!(iter.next(), Some((&3, &"crate")));
        assert_eq!(iter.next(), Some((&4, &"hello")));
        assert_eq!(iter.next(), Some((&5, &"iter")));
        assert_eq!(iter.next(), Some((&6, &"mod")));
        assert_eq!(iter.next(), Some((&7, &"tree")));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn preorder() {

        let tree = tree_setup();

        let mut iter = tree.preorder();

        assert_eq!(iter.next(), Some((&4, &"hello")));
        assert_eq!(iter.next(), Some((&2, &"world")));
        assert_eq!(iter.next(), Some((&1, &"rust")));
        assert_eq!(iter.next(), Some((&3, &"crate")));
        assert_eq!(iter.next(), Some((&6, &"mod")));
        assert_eq!(iter.next(), Some((&5, &"iter")));
        assert_eq!(iter.next(), Some((&7, &"tree")));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn postorder() {

        let tree = tree_setup();
        
        let mut iter = tree.postorder();

        assert_eq!(iter.next(), Some((&1, &"rust")));
        assert_eq!(iter.next(), Some((&3, &"crate")));
        assert_eq!(iter.next(), Some((&2, &"world")));
        assert_eq!(iter.next(), Some((&5, &"iter")));
        assert_eq!(iter.next(), Some((&7, &"tree")));
        assert_eq!(iter.next(), Some((&6, &"mod")));
        assert_eq!(iter.next(), Some((&4, &"hello")));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn values_mut() {

        let mut tree = tree_setup();

        for v in tree.values_mut() {
            if v == &"hello" {
                *v = "goodbye"
            }
        }
        assert!(tree.contains_val(&"goodbye"));
        assert!(!tree.contains_val(&"hello"));

        let mut iter_mut = tree.values_mut();

        assert_eq!(iter_mut.next(), Some(&mut "goodbye"));
        assert_eq!(iter_mut.next(), Some(&mut "world"));
        assert_eq!(iter_mut.next(), Some(&mut "mod"));
        assert_eq!(iter_mut.next(), Some(&mut "rust"));
        assert_eq!(iter_mut.next(), Some(&mut "crate"));
        assert_eq!(iter_mut.next(), Some(&mut "iter"));
        assert_eq!(iter_mut.next(), Some(&mut "tree"));
        assert_eq!(iter_mut.next(), None);

    }
    #[test]
    fn into_iterator() {

        let tree = tree_setup();

        let mut into_iter = tree.into_iter();

        assert_eq!(into_iter.next(), Some((4, "hello")));
        assert_eq!(into_iter.next(), Some((2, "world")));
        assert_eq!(into_iter.next(), Some((1, "rust")));
        assert_eq!(into_iter.next(), Some((3, "crate")));
        assert_eq!(into_iter.next(), Some((6, "mod")));
        assert_eq!(into_iter.next(), Some((5, "iter")));
        assert_eq!(into_iter.next(), Some((7, "tree")));
        assert_eq!(into_iter.next(), None);
    }
}
