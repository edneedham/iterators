type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug, Clone)]
pub struct BST<K, V> {
    pub root: Link<K, V>,
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

    pub fn inorder<'a>(&'a self) -> InOrderIter<'a, K, V> {
        InOrderIter {
            stack: Vec::new(),
            current: self.root.as_ref(),
        }
    }
}

struct PreOrderIter<K, V>(BST<K, V>);
struct PostOrderIter<K, V>(BST<K, V>);

pub struct InOrderIter<'a, K: Ord, V> {
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
}
