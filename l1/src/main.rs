use std::collections::HashMap;


#[derive(Debug, Clone)]
enum ListType {
    Simple,
    MoveToFront,
    Transpose,
    Count(HashMap<u32, u32>),
}

#[derive(Debug, Clone)]
struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
    list_type: ListType,
}

impl LinkedList {
    fn new(list_type: ListType) -> Self {
        LinkedList {
            head: None,
            list_type,
        }
    }

    /// Pops the first element from the list
    fn pop(&mut self) -> Option<u32> {
        if let Some(mut head) = self.head.take() {
            self.head = head.next.take();
            Some(head.value)
        } else {
            None
        }
    }

    fn access(&mut self, value: u32) -> u32 {
        match self.list_type {
            ListType::Simple => {
                self._simple_access(value)
            }
            ListType::MoveToFront => {
                self._mtf_access(value)
            }
            ListType::Transpose => {
                self._transpose_access(value)
            }
            ListType::Count(_) => {
                self._count_access(value)
            }
        }
    }

    /// Find if not found, add to the back return number of searched nodes
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

    /// Find, after found move to the front, else place at the back, then return number of searched nodes
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

    /// Find, after found move one place up then return number of searched nodes. if not found, add to the back
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
                // Swap the current node with the found node
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

    /// Find, after found increment the count, then return the count. If not found, add to the back it should be sorted descending
    fn _count_access(&mut self, value: u32, ) -> u32 {
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
                return searched_nodes;
            }
            searched_nodes += 1;
            current = &mut current.as_mut().unwrap().next;
        }
        let new_node = Box::new(Node { value, next: None });
        current.as_mut().unwrap().next = Some(new_node);
        counts.insert(value, 1);
        searched_nodes
    }
}

fn main() {
    for list_type in [ListType::Simple, ListType::MoveToFront, ListType::Transpose, ListType::Count(HashMap::new())] {
        println!("Testing list type: {:?}", list_type);
        let mut list = LinkedList::new(list_type);
        list.access(1);
        println!("List: {:?}", list);
        list.access(2);
        println!("List: {:?}", list);
        list.access(3);
        println!("List: {:?}", list);
        list.access(1);
        println!("List: {:?}", list);
        println!("Popped value: {:?}", list.pop());
        println!("Popped value: {:?}", list.pop());
        println!("Popped value: {:?}", list.pop());
        println!("Popped value: {:?}", list.pop());
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
            println!("Accessing: {:?}", value);
            println!("List: {:?}", list);
            assert_eq!(list.access(value), expected);
        }
        let pop_data: [Option<u32>; 4] = [Some(3), Some(1), Some(2), None];
        for expected in pop_data {
            assert_eq!(list.pop(), expected);
        }
    }
}
