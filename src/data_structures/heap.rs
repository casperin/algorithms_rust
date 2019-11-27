use std::cmp::Ordering;

/// Min heap implementation. Works either straight on values that can be compared (like numbers and
/// strings), or you can initiate it with a compare function in which case there are no
/// restrictions on the type of values this heap works with.
/// The heap also implements the iterator traits.
pub struct Heap<'a, T> {
    items: Vec<T>,
    compare: &'a dyn Fn(&T, &T) -> Ordering,
}

// Default comparing function. If the heap is created without a function to compare values, then
// this one is used.
fn compare<T: Ord>(a: &T, b: &T) -> Ordering {
    a.cmp(b)
}

impl<'a, T> Heap<'a, T> {
    pub fn new() -> Self
    where
        T: Ord,
    {
        Heap {
            items: Vec::new(),
            compare: &compare,
        }
    }

    /// Creates a new heap from the provided array.
    /// O(n log n)
    pub fn from_vec(items: Vec<T>) -> Self
    where
        T: Ord,
    {
        let mut heap = Heap {
            items: items,
            compare: &compare,
        };
        heap.heapify();
        heap
    }

    /// Create a new heap that uses the provided function to compare items.
    pub fn with_compare(compare: &'a dyn Fn(&T, &T) -> Ordering) -> Self {
        Heap {
            items: Vec::new(),
            compare: compare,
        }
    }

    pub fn from_vec_with_compare(items: Vec<T>, compare: &'a dyn Fn(&T, &T) -> Ordering) -> Self {
        let mut heap = Heap {
            items: items,
            compare: compare,
        };
        heap.heapify();
        heap
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Push new item onto the heap.
    /// O(log n)
    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.sift_up(self.items.len() - 1);
    }

    /// Let's you have a look at the lowest value without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    /// Pop the lowest value out of the heap.
    /// O(log n)
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

    /// Similar to `heap.push(item);` and then `heap.pop().unwrap();`. If you need to do both those
    /// though, then this function will be faster.
    /// O(log n)
    pub fn replace(&mut self, item: T) -> T {
        self.items.push(item); // we avoid the sifting up here
        self.pop().unwrap() // popping will balance it for us instead
    }

    /// Combines another heap of similar type with this one, consuming the heap passed in.
    /// O(n log n)
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

        if (self.compare)(&self.items[index], &self.items[parent_index]) == Ordering::Less {
            self.items.swap(index, parent_index);
            self.sift_up(parent_index);
        }
    }

    fn sift_down(&mut self, index: usize) {
        let mut smallest = index;
        let left = index * 2 + 1;
        let right = index * 2 + 2;
        let items = &self.items;

        if left < items.len() && (self.compare)(&items[left], &items[smallest]) == Ordering::Less {
            smallest = left;
        }

        if right < items.len() && (self.compare)(&items[right], &items[smallest]) == Ordering::Less
        {
            smallest = right;
        }

        if smallest != index {
            self.items.swap(index, smallest);
            self.sift_down(smallest);
        }
    }
}

pub struct IntoIter<'a, T>(Heap<'a, T>);

impl<'a, T> Iterator for Heap<'a, T> {
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
            let v: Vec<i64> = input.to_vec();
            let mut heap = Heap::from_vec(v);

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
    fn with_compare() {
        type K = (&'static str, i32);
        let mut heap = Heap::with_compare(&|a: &K, b: &K| a.1.cmp(&b.1));
        heap.push(("Alice", 3));
        heap.push(("Bob", 1));
        heap.push(("Carol", 2));
        assert_eq!(heap.next(), Some(("Bob", 1)));
        assert_eq!(heap.next(), Some(("Carol", 2)));
        assert_eq!(heap.next(), Some(("Alice", 3)));
    }

    #[test]
    fn from_vec_with_compare() {
        type K = (&'static str, i32);
        let v: Vec<K> = vec![("Alice", 3), ("Bob", 1), ("Carol", 2)];
        let mut heap = Heap::from_vec_with_compare(v, &|a: &K, b: &K| a.1.cmp(&b.1));
        assert_eq!(heap.next(), Some(("Bob", 1)));
        assert_eq!(heap.next(), Some(("Carol", 2)));
        assert_eq!(heap.next(), Some(("Alice", 3)));
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
