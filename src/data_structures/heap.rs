/// Min head.
pub struct Heap<T: Ord> {
    items: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Heap { items: Vec::new() }
    }

    pub fn from_vec(items: Vec<T>) -> Self {
        let mut heap = Heap { items: items };
        heap.heapify();
        heap
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.sift_up(self.items.len() - 1);
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let length = self.items.len();
        self.items.swap(0, length - 1);
        let min = self.items.pop();
        self.sift_down(0);
        min
    }

    pub fn replace(&mut self, item: T) -> T {
        self.items.push(item); // we avoid the sifting up here
        self.pop().unwrap() // popping will balance it for us instead
    }

    pub fn meld(&mut self, other_heap: Heap<T>) {
        for item in other_heap {
            self.push(item);
        }
    }

    fn heapify(&mut self) {
        for index in (0..self.items.len()).rev() {
            self.sift_down(index);
        }
    }

    fn sift_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent_index = (index - 1) / 2;

        if self.items[index] < self.items[parent_index] {
            self.items.swap(index, parent_index);
            self.sift_up(parent_index);
        }
    }

    fn sift_down(&mut self, index: usize) {
        let mut smallest = index;
        let left = index * 2 + 1;
        let right = index * 2 + 2;
        let items = &self.items;

        if left < items.len() && items[left] < items[smallest] {
            smallest = left;
        }

        if right < items.len() && items[right] < items[smallest] {
            smallest = right;
        }

        if smallest != index {
            self.items.swap(index, smallest);
            self.sift_down(smallest);
        }
    }
}

pub struct IntoIter<T: Ord>(Heap<T>);

impl<T: Ord> Iterator for Heap<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers;
    use super::Heap;

    #[test]
    fn random_numbers() {
        let tests = test_helpers::get_random_numbers();

        for i in 0..tests.len() {
            let (input, result, msg) = &tests[i];
            let mut heap = Heap::from_vec(input.to_vec());

            assert_eq!(heap.size(), input.len());

            for number in result {
                let peeked = heap.peek().unwrap().clone();
                let popped = heap.pop().unwrap();
                assert_eq!(peeked, popped, "{}th test: {}", i, msg);
                assert_eq!(&popped, number, "{}th test: {}", i, msg);
            }

            assert_eq!(heap.pop(), None);
        }
    }

    #[test]
    fn iter() {
        let mut heap = Heap::new();
        heap.push(2);
        heap.push(1);
        heap.push(3);
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(3));
    }

    #[test]
    fn into_iter() {
        let mut heap = Heap::new();
        heap.push(2);
        heap.push(1);
        heap.push(3);
        let mut iter = heap.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
    }

    #[test]
    fn replace() {
        let mut heap = Heap::from_vec(vec![2, 1]);
        assert_eq!(heap.replace(100), 1);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(100));
        assert!(heap.is_empty());
        assert_eq!(heap.replace(42), 42);
    }

    #[test]
    fn meld() {
        let mut heap1 = Heap::from_vec(vec![3, 4, 1]);
        let heap2 = Heap::from_vec(vec![2, 5, 0]);
        heap1.meld(heap2);
        for n in 0..6 {
            assert_eq!(heap1.pop(), Some(n));
        }
    }
}
