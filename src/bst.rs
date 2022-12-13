use core::marker::PhantomData;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug, Clone)]
pub struct BST<K, V> {
    root: Link<K, V>,
}

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    pub left: Link<K, V>,
    pub right: Link<K, V>,
    pub key: K,
    pub value: V,
}

impl<K: Clone + Ord, V: Clone> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            key,
            value,
        }
    }
}

struct PreOrderIter<K, V>(BST<K, V>);
struct PostOrderIter<K, V>(BST<K, V>);

struct InOrderIter<K: Ord, V> {
    stack: Vec<Link<K, V>>,
    current: Link<K, V>,
}

impl<K: Ord, V> Iterator for InOrderIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        loop {
            if let Some(current) = &self.current {
                self.stack.push(self.current);
                self.current = current.left;
            } else {
                if let Some(node) = self.stack.pop() {
                    let current = &node.unwrap();
                    let result = (current.key, current.value);
                    self.current = current.right;
                    return Some(result);
                } else {
                    return None;
                }
            }
        }
    }
}

impl<K: Ord + PartialEq, V> BST<K, V> {
    pub fn insert(&mut self, input: Node<K, V>) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            if input.key > node.key {
                current = &mut node.right;
            } else {
                current = &mut node.left;
            } 
        }
        *current = Some(Box::new(input));
    }

    pub fn inorder(&self) -> InOrderIter<K, V> {
        InOrderIter {
            stack: Vec::new(),
            current: self.root,
            marker: PhantomData,
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}
