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

impl<K: Ord + PartialEq, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            key,
            value,
        }
    }
}

impl<K: Ord + PartialEq, V: PartialEq> BST<K, V> {
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

    // Breadth-first search to get the height
    pub fn height(&self) -> usize {
        let mut queue = VecDeque::new();
        let mut height = 0;
        if self.root.is_none() {
            return height;
        } else {
            queue.push_back(self.root.as_ref());
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
            height
        }
    }

    pub fn inorder<'a>(&'a self) -> InOrderIter<'a, K, V> {
        InOrderIter {
            stack: vec![self.root.as_ref()],
        }
    }
    pub fn preorder<'a>(&'a self) -> PreOrderIter<'a, K, V> {
        PreOrderIter {
            stack: vec![self.root.as_ref()],
            current: self.root.as_ref(),
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
            stack: Vec::new(),
            current: self.root.as_mut(),
        }
    }

    pub fn contains_val(&self, value: &V) -> bool {
        let mut stack = vec![self.root.as_ref()];
        while let Some(node) = stack.pop() {
            let node = node.unwrap();
            if &node.value == value {
                return true;
            } else if node.right.is_some() && node.left.is_some() {
                stack.push(node.right.as_ref());
                stack.push(node.left.as_ref());
            } else if node.right.is_some() {
                stack.push(node.right.as_ref());
            } else if node.left.is_some() {
                stack.push(node.left.as_ref());
            } else {
                break;
            }
        }
        false
    }
}

// We want to build up a stack of nodes in order 
// Then we return the stack of nodes
// We then implement the iterator on the stack of nodes
// returning each item (the keys and values of each node) from the stack
// on each call to next
//
//
// two choices 
// 
// either we add the root and all left nodes to the stack
// and then when we call next on the iterator implementation we add
// the right nodes to the stack at this time
//
// or we traverse all the nodes in breadth first fashion adding them all to
// the stack during into_iter
pub struct IntoIter<K: Ord, V> {
    stack: Vec<Option<Box<Node<K, V>>>>,
}

impl<K: Ord, V> IntoIterator for BST<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: vec![self.root] }
    }
}

impl<K: Ord, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.stack.pop() {
            let current = current.unwrap();
            if current.right.is_some() {
                self.stack.push(current.right)
            } 

            if current.left.is_some() {
                self.stack.push(current.left)
            }
        Some((current.key, current.value))
        } else {
            None
        }
    }
}

pub struct InOrderIter<'a, K: Ord, V> {
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
}

impl<'a, K: Ord, V> Iterator for InOrderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    // The left child is added to the stack last on each call
    // so the left child is popped from the stack first and therefore
    // visited first.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.stack.pop() {
            let current = current.unwrap();
            if current.right.is_some() {
                self.stack.push(current.right.as_ref())
            } 

            if current.left.is_some() {
                self.stack.push(current.left.as_ref())
            }
        Some((&current.key, &current.value))
        } else {
            None
        }

    }
}

pub struct PreOrderIter<'a, K: Ord, V> {
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
    current: Option<&'a Box<Node<K, V>>>,
}

impl<'a, K: Ord, V> Iterator for PreOrderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current) = self.current {
                let result = (&current.key, &current.value);
                self.current = current.left.as_ref();
                self.stack.push(self.current);
                return Some(result);
            } else {
                if let Some(node) = self.stack.pop() {
                    if node.is_some() {
                        let current = node.unwrap();
                        self.current = current.right.as_ref();
                        if self.current.is_some() {
                            self.stack.push(self.current);

                        }
                    }
                } else {
                    return None;
                }
            }
        }
    }
}

pub struct PostOrderIter<'a, K: Ord, V>{
    stack: Vec<Option<&'a Box<Node<K, V>>>>,
    current: Option<&'a Box<Node<K, V>>>,
}

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

pub struct ValuesMut<'a, K: Ord, V> {
    stack: Vec<Option<&'a mut Box<Node<K, V>>>>,
    current: Option<&'a mut Box<Node<K, V>>>,
}

impl<'a, K: Ord, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let node = node.unwrap();
            if let Some(right) = &mut node.right {
                self.stack.push(Some(right));
            }
            if let Some(left) = &mut node.left {
                self.stack.push(Some(left));
            }
            return Some(&mut node.value);
        }
        let current = self.current.take();
        if let Some(current) = current {
            self.stack.push(Some(&mut *current));
            self.next()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tree_setup() -> BST<i32, &'static str> {
        let node = Node::new(5, "hello");
        let node2 = Node::new(8, "world");
        let node3 = Node::new(11, "rust");
        let node4 = Node::new(2, "crate");
        let node5 = Node::new(4, "mod");
        let node6 = Node::new(7, "iter");
        let node7 = Node::new(10, "tree");

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
    fn tree_has_children() {
        let tree = tree_setup();
        assert!(tree.root.is_some());
        assert!(tree.root.as_ref().unwrap().right.is_some());
        assert!(tree.root.as_ref().unwrap().left.is_some());
    }
    #[test]
    fn height() {
        let tree = tree_setup();
        assert_eq!(tree.height(), 4);
    }
    #[test]
    fn mutating_a_value() {
        let mut tree = tree_setup();
        for v in tree.values_mut() {
            if v == &"hello" {
                *v = "goodbye"
            }
        }
        assert!(tree.contains_val(&"goodbye"));
        assert!(!tree.contains_val(&"hello"));
    }
    #[test]
    fn into_iterator() {
        let tree = tree_setup();
        let mut iter = tree.into_iter();
        assert_eq!(Some((5, "hello")), iter.next());
    }
}
