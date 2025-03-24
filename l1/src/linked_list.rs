use std::collections::HashMap;

/// Enum representing the type of linked list.
#[derive(Debug, Clone)]
pub enum ListType {
    /// A simple linked list with no special behavior.
    Simple,
    /// A linked list that moves accessed elements to the front.
    MoveToFront,
    /// A linked list that swaps accessed elements with their predecessor.
    Transpose,
    /// A linked list that maintains elements sorted by access count.
    Count(HashMap<u32, u32>),
}

/// A node in the linked list.
#[derive(Debug, Clone)]
struct Node {
    /// The value stored in the node.
    value: u32,
    /// The next node in the list.
    next: Option<Box<Node>>,
}

/// A linked list with various access strategies.
#[derive(Debug)]
pub struct LinkedList {
    /// The head of the linked list.
    head: Option<Box<Node>>,
    /// The type of the linked list.
    list_type: ListType,
}

impl LinkedList {
    /// Creates a new linked list of the specified type.
    ///
    /// # Arguments
    ///
    /// * `list_type` - The type of the linked list.
    ///
    /// # Returns
    ///
    /// A new `LinkedList` instance.
    pub fn new(list_type: ListType) -> Self {
        LinkedList {
            head: None,
            list_type,
        }
    }

    /// Removes and returns the first element from the list.
    ///
    /// # Returns
    ///
    /// The value of the first element, or `None` if the list is empty.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        if let Some(mut head) = self.head.take() {
            self.head = head.next.take();
            Some(head.value)
        } else {
            None
        }
    }

    /// Accesses a value in the list, applying the behavior of the list type.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to access.
    ///
    /// # Returns
    ///
    /// The number of nodes searched to find the value.
    pub fn access(&mut self, value: u32) -> u32 {
        match self.list_type {
            ListType::Simple => self._simple_access(value),
            ListType::MoveToFront => self._mtf_access(value),
            ListType::Transpose => self._transpose_access(value),
            ListType::Count(_) => self._count_access(value),
        }
    }

    /// Accesses a value in a simple list. If not found, adds it to the back.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to access.
    ///
    /// # Returns
    ///
    /// The number of nodes searched to find the value.
    fn _simple_access(&mut self, value: u32) -> u32 {
        match self.head {
            None => {
                let new_node = Box::new(Node { value, next: None });
                self.head = Some(new_node);
                return 0;
            }
            Some(ref mut head) => {
                if head.value == value {
                    return 1;
                }
            }
        }
        let mut current = &mut self.head;
        let mut searched_nodes = 1;
        while let Some(node) = current.as_ref().unwrap().next.as_ref() {
            searched_nodes += 1;
            if node.value == value {
                return searched_nodes;
            }
            current = &mut current.as_mut().unwrap().next;
        }
        let new_node = Box::new(Node { value, next: None });
        current.as_mut().unwrap().next = Some(new_node);
        searched_nodes
    }

    /// Accesses a value in a Move-To-Front list. Moves the value to the front if found.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to access.
    ///
    /// # Returns
    ///
    /// The number of nodes searched to find the value.
    fn _mtf_access(&mut self, value: u32) -> u32 {
        match self.head {
            None => {
                let new_node = Box::new(Node { value, next: None });
                self.head = Some(new_node);
                return 0;
            }
            Some(ref mut head) => {
                if head.value == value {
                    return 1;
                }
            }
        }
        let mut current = &mut self.head;
        let mut searched_nodes = 1;
        while let Some(node) = current.as_ref().unwrap().next.as_ref() {
            if node.value == value {
                let mut found_node = current.as_mut().unwrap().next.take();
                let head = self.head.take();
                found_node.as_mut().unwrap().next = head;
                self.head = found_node;
                return searched_nodes + 1;
            }
            searched_nodes += 1;
            current = &mut current.as_mut().unwrap().next;
        }
        let new_node = Box::new(Node { value, next: None });
        current.as_mut().unwrap().next = Some(new_node);
        searched_nodes
    }

    /// Accesses a value in a Transpose list. Swaps the value with its predecessor if found.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to access.
    ///
    /// # Returns
    ///
    /// The number of nodes searched to find the value.
    fn _transpose_access(&mut self, value: u32) -> u32 {
        match self.head {
            None => {
                let new_node = Box::new(Node { value, next: None });
                self.head = Some(new_node);
                return 0;
            }
            Some(ref mut head) => {
                if head.value == value {
                    return 1;
                }
            }
        }
        let mut current = &mut self.head;
        let mut searched_nodes = 1;
        while let Some(node) = current.as_ref().unwrap().next.as_ref() {
            if node.value == value {
                let mut found_node = current.as_mut().unwrap().next.take();
                let temp = current.take();
                if let Some(mut temp_node) = temp {
                    temp_node.next = found_node.as_mut().unwrap().next.take();
                    found_node.as_mut().unwrap().next = Some(temp_node);
                }
                *current = Some(found_node.unwrap());
                return searched_nodes + 1;
            }
            searched_nodes += 1;
            current = &mut current.as_mut().unwrap().next;
        }
        let new_node = Box::new(Node { value, next: None });
        current.as_mut().unwrap().next = Some(new_node);
        searched_nodes
    }

    /// Accesses a value in a Count list. Increments the count and reorders the list.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to access.
    ///
    /// # Returns
    ///
    /// The number of nodes searched to find the value.
    fn _count_access(&mut self, value: u32) -> u32 {
        let counts = match &mut self.list_type {
            ListType::Count(counts) => counts,
            _ => panic!("Invalid list type"),
        };
        match self.head {
            None => {
                let new_node = Box::new(Node { value, next: None });
                self.head = Some(new_node);
                counts.insert(value, 1);
                return 0;
            }
            Some(ref mut head) => {
                if head.value == value {
                    *counts.entry(value).or_insert(0) += 1;
                    return 1;
                }
            }
        }
        let mut current = &mut self.head;
        let mut searched_nodes = 1;
        while let Some(node) = current.as_ref().unwrap().next.as_ref() {
            if node.value == value {
                *counts.entry(value).or_insert(0) += 1;
                self._reorder_by_count();
                return searched_nodes + 1;
            }
            searched_nodes += 1;
            current = &mut current.as_mut().unwrap().next;
        }
        let new_node = Box::new(Node { value, next: None });
        current.as_mut().unwrap().next = Some(new_node);
        counts.insert(value, 1);
        searched_nodes
    }

    /// Reorders the list based on descending access counts.
    fn _reorder_by_count(&mut self) {
        let counts = match &mut self.list_type {
            ListType::Count(counts) => counts,
            _ => panic!("Invalid list type"),
        };
        let mut swapped;
        loop {
            swapped = false;
            let mut current = &mut self.head;
            while let Some(node) = current.as_ref().unwrap().next.as_ref() {
                let current_value = current.as_ref().unwrap().value;
                let next_value = node.value;
                if counts[&current_value] < counts[&next_value] {
                    let mut found_node = current.as_mut().unwrap().next.take();
                    let temp = current.take();
                    if let Some(mut temp_node) = temp {
                        temp_node.next = found_node.as_mut().unwrap().next.take();
                        found_node.as_mut().unwrap().next = Some(temp_node);
                    }
                    *current = Some(found_node.unwrap());
                    swapped = true;
                }
                current = &mut current.as_mut().unwrap().next;
            }
            if !swapped {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_access() {
        let mut list = LinkedList::new(ListType::Simple);
        let access_data = [(1, 0), (2, 1), (3, 2), (3, 3)];
        for (value, expected) in access_data {
            assert_eq!(list.access(value), expected);
        }
        let pop_data: [Option<u32>; 4] = [Some(1), Some(2), Some(3), None];
        for expected in pop_data {
            assert_eq!(list.pop(), expected);
        }
    }

    #[test]
    fn test_mtf_access() {
        let mut list = LinkedList::new(ListType::MoveToFront);
        let access_data = [(1, 0), (2, 1), (3, 2), (1, 1), (3, 3), (3, 1)];
        for (value, expected) in access_data {
            assert_eq!(list.access(value), expected);
        }
        let pop_data: [Option<u32>; 4] = [Some(3), Some(1), Some(2), None];
        for expected in pop_data {
            assert_eq!(list.pop(), expected);
        }
    }

    #[test]
    fn test_transpose_access() {
        let mut list = LinkedList::new(ListType::Transpose);
        let access_data = [(1, 0), (2, 1), (3, 2), (1, 1), (3, 3), (3, 2)];
        for (value, expected) in access_data {
            assert_eq!(list.access(value), expected);
        }
        let pop_data: [Option<u32>; 4] = [Some(3), Some(1), Some(2), None];
        for expected in pop_data {
            assert_eq!(list.pop(), expected);
        }
    }

    #[test]
    fn test_count_access() {
        let mut list = LinkedList::new(ListType::Count(HashMap::new()));
        let access_data = [(1, 0), (2, 1), (3, 2), (1, 1), (3, 3), (3, 2)];
        for (value, expected) in access_data {
            assert_eq!(list.access(value), expected);
        }
        let pop_data: [Option<u32>; 4] = [Some(3), Some(1), Some(2), None];
        for expected in pop_data {
            assert_eq!(list.pop(), expected);
        }
    }
}
