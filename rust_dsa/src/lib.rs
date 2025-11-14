pub mod stack {
    /// A simple stack data structure backed by a `Vec`.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct Stack<T> {
        data: Vec<T>,
    }

    impl<T> Stack<T> {
        /// Creates an empty stack.
        pub fn new() -> Self {
            Self { data: Vec::new() }
        }

        /// Returns `true` if the stack has no elements.
        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        /// Returns the number of elements in the stack.
        pub fn len(&self) -> usize {
            self.data.len()
        }

        /// Pushes an element onto the stack.
        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        /// Removes the top element from the stack and returns it.
        pub fn pop(&mut self) -> Option<T> {
            self.data.pop()
        }

        /// Returns a reference to the top element without removing it.
        pub fn peek(&self) -> Option<&T> {
            self.data.last()
        }
    }
}

pub mod queue {
    use std::collections::VecDeque;

    /// A queue data structure using a double-ended queue for efficient
    /// insertion/removal at both ends.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct Queue<T> {
        data: VecDeque<T>,
    }

    impl<T> Queue<T> {
        /// Creates an empty queue.
        pub fn new() -> Self {
            Self {
                data: VecDeque::new(),
            }
        }

        /// Returns `true` if the queue is empty.
        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        /// Returns the number of elements in the queue.
        pub fn len(&self) -> usize {
            self.data.len()
        }

        /// Adds an element to the back of the queue.
        pub fn enqueue(&mut self, value: T) {
            self.data.push_back(value);
        }

        /// Removes an element from the front of the queue and returns it.
        pub fn dequeue(&mut self) -> Option<T> {
            self.data.pop_front()
        }

        /// Returns a reference to the front element without removing it.
        pub fn peek(&self) -> Option<&T> {
            self.data.front()
        }
    }
}

pub mod linked_list {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Node<T> {
        pub value: T,
        pub next: Option<Box<Node<T>>>,
    }

    /// A singly linked list.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
        len: usize,
    }

    impl<T> LinkedList<T> {
        /// Creates an empty linked list.
        pub fn new() -> Self {
            Self { head: None, len: 0 }
        }

        /// Returns the number of nodes in the list.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if the list contains no elements.
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// Adds a value to the front of the list.
        pub fn push_front(&mut self, value: T) {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            self.len += 1;
        }

        /// Removes the first element and returns it.
        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                self.len -= 1;
                node.value
            })
        }

        /// Returns a reference to the first element.
        pub fn front(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.value)
        }

        /// Iterates over references to the list's values.
        pub fn iter(&self) -> LinkedListIter<'_, T> {
            LinkedListIter {
                next: self.head.as_deref(),
            }
        }
    }

    /// Iterator over references to the values in the linked list.
    pub struct LinkedListIter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for LinkedListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.value
            })
        }
    }
}

pub mod bst {
    /// A binary search tree node.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Node<T> {
        pub value: T,
        pub left: Option<Box<Node<T>>>,
        pub right: Option<Box<Node<T>>>,
    }

    impl<T> Node<T>
    where
        T: Ord,
    {
        fn new(value: T) -> Self {
            Self {
                value,
                left: None,
                right: None,
            }
        }

        /// Inserts a value into the subtree rooted at this node. Returns
        /// `true` if the value was newly inserted.
        pub fn insert(&mut self, value: T) -> bool {
            if value < self.value {
                match self.left {
                    Some(ref mut left) => left.insert(value),
                    None => {
                        self.left = Some(Box::new(Node::new(value)));
                        true
                    }
                }
            } else if value > self.value {
                match self.right {
                    Some(ref mut right) => right.insert(value),
                    None => {
                        self.right = Some(Box::new(Node::new(value)));
                        true
                    }
                }
            } else {
                false
            }
        }

        /// Returns `true` if the subtree contains the value.
        pub fn contains(&self, value: &T) -> bool {
            if value == &self.value {
                true
            } else if value < &self.value {
                self.left
                    .as_ref()
                    .map(|left| left.contains(value))
                    .unwrap_or(false)
            } else {
                self.right
                    .as_ref()
                    .map(|right| right.contains(value))
                    .unwrap_or(false)
            }
        }

        /// Performs an in-order traversal, pushing values into the provided vector.
        pub fn inorder<'a>(&'a self, acc: &mut Vec<&'a T>) {
            if let Some(left) = self.left.as_deref() {
                left.inorder(acc);
            }
            acc.push(&self.value);
            if let Some(right) = self.right.as_deref() {
                right.inorder(acc);
            }
        }
    }

    /// A binary search tree wrapper.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct BinarySearchTree<T> {
        root: Option<Box<Node<T>>>,
        len: usize,
    }

    impl<T> BinarySearchTree<T>
    where
        T: Ord,
    {
        /// Creates an empty tree.
        pub fn new() -> Self {
            Self { root: None, len: 0 }
        }

        /// Returns `true` if the tree contains no nodes.
        pub fn is_empty(&self) -> bool {
            self.root.is_none()
        }

        /// Returns the number of nodes in the tree.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Inserts a value into the tree.
        pub fn insert(&mut self, value: T) {
            let inserted = match self.root {
                Some(ref mut node) => node.insert(value),
                None => {
                    self.root = Some(Box::new(Node::new(value)));
                    true
                }
            };
            if inserted {
                self.len += 1;
            }
        }

        /// Returns `true` if the tree contains the value.
        pub fn contains(&self, value: &T) -> bool {
            self.root
                .as_ref()
                .map(|node| node.contains(value))
                .unwrap_or(false)
        }

        /// Returns the values in the tree in sorted order.
        pub fn inorder(&self) -> Vec<&T> {
            let mut values = Vec::with_capacity(self.len);
            if let Some(root) = self.root.as_deref() {
                root.inorder(&mut values);
            }
            values
        }
    }
}

pub mod sorting {
    /// Performs merge sort on a mutable slice.
    pub fn merge_sort<T: Ord + Clone>(data: &mut [T]) {
        let len = data.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        let mut left = data[..mid].to_vec();
        let mut right = data[mid..].to_vec();
        merge_sort(&mut left);
        merge_sort(&mut right);

        let mut left_iter = left.into_iter();
        let mut right_iter = right.into_iter();
        let mut left_peek = left_iter.next();
        let mut right_peek = right_iter.next();

        for slot in data.iter_mut() {
            match (&left_peek, &right_peek) {
                (Some(l), Some(r)) => {
                    if l <= r {
                        *slot = l.clone();
                        left_peek = left_iter.next();
                    } else {
                        *slot = r.clone();
                        right_peek = right_iter.next();
                    }
                }
                (Some(l), None) => {
                    *slot = l.clone();
                    left_peek = left_iter.next();
                }
                (None, Some(r)) => {
                    *slot = r.clone();
                    right_peek = right_iter.next();
                }
                (None, None) => break,
            }
        }
    }

    /// Performs quick sort on a mutable slice.
    pub fn quick_sort<T: Ord>(data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        quick_sort_recursive(data, 0, data.len() - 1);
    }

    fn quick_sort_recursive<T: Ord>(data: &mut [T], low: usize, high: usize) {
        if low < high {
            let pivot_index = partition(data, low, high);
            if pivot_index > 0 {
                quick_sort_recursive(data, low, pivot_index - 1);
            }
            quick_sort_recursive(data, pivot_index + 1, high);
        }
    }

    fn partition<T: Ord>(data: &mut [T], low: usize, high: usize) -> usize {
        let mut store_index = low;
        for i in low..high {
            if data[i] <= data[high] {
                data.swap(i, store_index);
                store_index += 1;
            }
        }
        data.swap(store_index, high);
        store_index
    }
}

pub mod search {
    /// Performs binary search on a sorted slice. Returns the index of the
    /// target if found, or `None` otherwise.
    pub fn binary_search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
        let mut low = 0usize;
        let mut high = data.len();

        while low < high {
            let mid = (low + high) / 2;
            if &data[mid] == target {
                return Some(mid);
            } else if &data[mid] < target {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        None
    }
}

pub mod graph {
    use std::collections::{HashMap, HashSet, VecDeque};

    /// A simple directed graph represented by an adjacency list.
    #[derive(Debug, Default, Clone)]
    pub struct Graph<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        adjacency: HashMap<T, Vec<T>>,
    }

    impl<T> Graph<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        /// Creates an empty graph.
        pub fn new() -> Self {
            Self {
                adjacency: HashMap::new(),
            }
        }

        /// Adds a directed edge from `from` to `to`.
        pub fn add_edge(&mut self, from: T, to: T) {
            self.adjacency.entry(from).or_default().push(to);
        }

        /// Returns the neighbors of a node.
        pub fn neighbors(&self, node: &T) -> Option<&[T]> {
            self.adjacency.get(node).map(|v| v.as_slice())
        }

        /// Performs breadth-first search from `start`, returning the order of
        /// visitation.
        pub fn bfs(&self, start: T) -> Vec<T> {
            let mut visited = HashSet::new();
            let mut order = Vec::new();
            let mut queue = VecDeque::new();

            queue.push_back(start.clone());
            visited.insert(start.clone());

            while let Some(node) = queue.pop_front() {
                order.push(node.clone());
                if let Some(neighbors) = self.adjacency.get(&node) {
                    for neighbor in neighbors {
                        if visited.insert(neighbor.clone()) {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }

            order
        }

        /// Performs depth-first search from `start`, returning the order of
        /// visitation.
        pub fn dfs(&self, start: T) -> Vec<T> {
            let mut visited = HashSet::new();
            let mut order = Vec::new();
            self.dfs_recursive(&start, &mut visited, &mut order);
            order
        }

        fn dfs_recursive(&self, node: &T, visited: &mut HashSet<T>, order: &mut Vec<T>)
        where
            T: Clone,
        {
            if !visited.insert(node.clone()) {
                return;
            }
            order.push(node.clone());
            if let Some(neighbors) = self.adjacency.get(node) {
                for neighbor in neighbors {
                    self.dfs_recursive(neighbor, visited, order);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bst::BinarySearchTree;
    use super::graph::Graph;
    use super::linked_list::LinkedList;
    use super::queue::Queue;
    use super::search::binary_search;
    use super::sorting::{merge_sort, quick_sort};
    use super::stack::Stack;

    #[test]
    fn stack_operations() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }

    #[test]
    fn queue_operations() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert!(queue.is_empty());
    }

    #[test]
    fn linked_list_usage() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.front(), Some(&2));
        let values: Vec<_> = list.iter().copied().collect();
        assert_eq!(values, vec![2, 1]);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn bst_operations() {
        let mut bst = BinarySearchTree::new();
        assert!(bst.is_empty());
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        assert_eq!(bst.len(), 4);
        assert!(bst.contains(&7));
        assert!(!bst.contains(&4));
        assert_eq!(bst.inorder(), vec![&1, &3, &5, &7]);
    }

    #[test]
    fn merge_sort_works() {
        let mut data = vec![3, 1, 4, 1, 5, 9];
        merge_sort(&mut data);
        assert_eq!(data, vec![1, 1, 3, 4, 5, 9]);
    }

    #[test]
    fn quick_sort_works() {
        let mut data = vec![10, -1, 2, 5, 0];
        quick_sort(&mut data);
        assert_eq!(data, vec![-1, 0, 2, 5, 10]);
    }

    #[test]
    fn binary_search_finds_values() {
        let data = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search(&data, &7), Some(3));
        assert_eq!(binary_search(&data, &4), None);
    }

    #[test]
    fn graph_traversals() {
        let mut graph = Graph::new();
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");
        graph.add_edge("B", "D");
        graph.add_edge("C", "D");

        let mut bfs_order = graph.bfs("A");
        assert_eq!(bfs_order.remove(0), "A");
        assert!(bfs_order.contains(&"B"));
        assert!(bfs_order.contains(&"C"));
        assert!(bfs_order.contains(&"D"));

        let dfs_order = graph.dfs("A");
        assert_eq!(dfs_order.first(), Some(&"A"));
        assert!(dfs_order.contains(&"D"));
    }
}
