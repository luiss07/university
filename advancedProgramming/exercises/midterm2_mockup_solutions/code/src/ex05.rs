//Implement a doubly linked list
//Create the necessary structs to represent it
//- `Node<T>` with an element of type `T` and two fields, `prev` and `next`, both of type `Option<Rc<RefCell<Node<T>>>>`.
//- `List<T>` with two fields, `head` and `tail`, both of type `Option<Rc<RefCell<Node<T>>>>`, and a size field of type `usize`.
//
//Implement the following traits for `Node<T>`:
//- `PartialEq` that compares the elements of two nodes.
//- `Display` that prints the element of a node.
//
//Implement the following traits for `List<T>`:
//- `PartialEq` that checks if two lists are equal, by comparing the elements of the nodes, one by one.
//- `Debug` that prints the elements of the list.
//
//Implement the following methods for `List<T>`:
//- `new()` that creates a new empty list.
//- `print_list(&self)` that prints the elements of the list.
//- `push(&mut self, element: T)` that adds an element to the front of the list.
//- `pop(&mut self) -> Option<T>` that removes an element from the front of the list.
//- `push_back(&mut self, element: T)` that adds an element to the back of the list.
//- `pop_back(&mut self) -> Option<T>` that removes an element from the back of the list.
//- `print_list(&self)` that prints the elements of the list.

use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::*;

#[derive(Debug)]
struct Node<T> {
    element: T,
    prev: Link<T>,
    next: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut currently_head = self.head.clone();
        while let Some(node) = currently_head {
            write!(f, "{}", node.borrow().element)?;
            currently_head = node.borrow().next.clone();
        }
        Ok(())
    }
}

impl<T: Display> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut currently_head = self.head.clone();
        while let Some(node) = currently_head {
            write!(f, "{}", node.borrow().element)?;
            currently_head = node.borrow().next.clone();
        }
        Ok(())
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut currently_head = self.head.clone();
        let mut currently_other_head = other.head.clone();
        while let Some(node) = currently_head {
            if let Some(other_node) = currently_other_head {
                if node.borrow().element != other_node.borrow().element {
                    return false;
                }
                currently_head = node.borrow().next.clone();
                currently_other_head = other_node.borrow().next.clone();
            } else {
                return false;
            }
        }
        if currently_other_head.is_some() {
            return false;
        }
        true
    }
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            element: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T: Clone> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.head.take() {
            Some(past_head) => {
                past_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(past_head.clone());
                self.head = Some(new_node);
                self.size += 1;
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
                self.size = 1;
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head_node) => {
                head_node.borrow_mut().prev = None;
                let returned_value = head_node.borrow().element.clone();
                match head_node.clone().borrow_mut().next.take() {
                    Some(element) => {
                        element.borrow_mut().prev = None;
                        self.head = Some(element.clone());
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                head_node.borrow_mut().next = None;
                self.size -= 1;
                return Some(returned_value);
            }
            None => None,
        }
    }

    fn push_back(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.tail.take() {
            Some(past_tail) => {
                past_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(past_tail.clone());
                self.tail = Some(new_node);
                self.size += 1;
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node.clone());
                self.size = 1;
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail_node) => {
                tail_node.borrow_mut().next = None;
                let returned_value = tail_node.borrow().element.clone();
                match tail_node.clone().borrow_mut().prev.take() {
                    Some(element) => {
                        element.borrow_mut().next = None;
                        self.tail = Some(element.clone());
                    }
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                }
                tail_node.borrow_mut().prev = None;
                self.size -= 1;
                return Some(returned_value);
            }
            None => None,
        }
    }
}

impl<T: Display> List<T> {
    fn print_list(&self) {
        let mut currently_head = self.head.clone();
        while let Some(node) = currently_head {
            println!("{}", node.borrow().element);
            currently_head = node.borrow().next.clone();
        }
    }
}

// ---

#[test]
fn print_linked_list() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.print_list();
}
/*
MARK 1
3
2
1
*/

#[test]
fn push_linked_list() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.print_list();
    debug_assert_eq!(list.size, 3);
    debug_assert_eq!(list.head.clone().unwrap().borrow().element, 3);
    debug_assert_eq!(list.tail.clone().unwrap().borrow().element, 1);
}
/*
MARK 1
3
2
1
*/

#[test]
fn pop_linked_list() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    debug_assert_eq!(list.pop(), Some(3));
    list.print_list();
    debug_assert_eq!(list.pop(), Some(2));
    list.print_list();
    debug_assert_eq!(list.pop(), Some(1));
    list.print_list();
    debug_assert_eq!(list.pop(), None);
    list.print_list();
    debug_assert_eq!(list.size, 0);
    debug_assert_eq!(list.head, None);
    debug_assert_eq!(list.tail, None);
}
/*
MARK 3
2
1
1
 */

#[test]
fn push_back_linked_list() {
    let mut list: List<i32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    debug_assert_eq!(list.size, 3);
    println!("{}", list.size);
    list.print_list();
    debug_assert_eq!(list.head.clone().unwrap().borrow().element, 1);
    println!("{}", list.head.clone().unwrap().borrow().element);
    debug_assert_eq!(list.tail.clone().unwrap().borrow().element, 3);
    println!("{}", list.tail.clone().unwrap().borrow().element);
}
/*
MARK 2
3
1
2
3
1
3
*/

#[test]
fn pop_back_linked_list() {
    let mut list: List<i32> = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(3));
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(2));
    list.print_list();
    debug_assert_eq!(list.pop_back(), Some(1));
    list.print_list();
    debug_assert_eq!(list.pop_back(), None);
    debug_assert_eq!(list.size, 0);
    debug_assert_eq!(list.head, None);
    println!("{:?}", list.head);
    debug_assert_eq!(list.tail, None);
    println!("{:?}", list.tail);
}
/*
MARK 3
1
2
3
1
2
1
None
None
*/
