// I have been unable to figure this out yet. Neither have I been able to find a version online
// that actually compiles.
// By now I feel I understand BST's the best out of any structure or algorithm, simply because of
// the time I have spent trying to implement it.
// For now, I will move on to look at graphs. Hopefully I will at some point be able to come back
// and finish this.

/*
use std::cmp::Ordering;

pub struct BST<T: Ord + Copy>
where
    T: std::fmt::Debug,
{
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Ord + Copy>
where
    T: std::fmt::Debug,
{
    key: T,
    left: Link<T>,
    right: Link<T>,
}

fn create_node<T: Ord + Copy>(key: T) -> Option<Box<Node<T>>>
where
    T: std::fmt::Debug,
{
    Some(Box::new(Node {
        key: key,
        left: None,
        right: None,
    }))
}

impl<T: Ord + Copy> BST<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        BST { head: None }
    }

    pub fn insert<'a>(&'a mut self, key: &'a T) {
        let mut head = &self.head;

        while let Some(h) = head {
            head = match h.key.cmp(&key) {
                Ordering::Less => &h.left,
                Ordering::Greater => &h.right,
                Ordering::Equal => {
                    return; // noop
                }
            };
        }

        *head = create_node(*key);
    }

    pub fn search(&self, key: T) -> Option<T> {
        self.search_node(self.head.as_ref(), key)
    }

    fn search_node(&self, node: Option<&Box<Node<T>>>, key: T) -> Option<T> {
        let node = node?;
        match node.key.cmp(&key) {
            Ordering::Equal => Some(node.key),
            Ordering::Less => self.search_node(node.right.as_ref(), key),
            Ordering::Greater => self.search_node(node.left.as_ref(), key),
        }
    }

    pub fn print(&self) {
        self.print_node(self.head.as_ref());
    }

    fn print_node(&self, n: Option<&Box<Node<T>>>) {
        match n {
            None => {
                println!("NOTHING");
            }
            Some(n) => {
                println!("---> {:?}", n.key);
                self.print_node(n.left.as_ref());
                self.print_node(n.right.as_ref());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BST;

    #[test]
    fn simple() {
        let mut bst = BST::new();

        bst.insert(&10);
        bst.insert(&20);
        bst.insert(&30);

        bst.print();

        assert_eq!(bst.search(2), Some(20));
    }
}
*/
