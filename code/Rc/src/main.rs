use std::{
    io::Lines,
    rc::{Rc, Weak},
};
fn main() {
    let name = Rc::new(format!("Rom"));
    let name_rc = Rc::clone(&name);
    let name_rcc = name.clone();
    // Rc -> Weak
    let name_weak = Rc::downgrade(&name);
    println!("{}, {}, {}", name, name_rc, name_rcc);
    println!("{}", Rc::strong_count(&name));
    drop(name);
    println!("{}", Rc::strong_count(&name_rc));
    drop(name_rc);
    println!("{}", Rc::strong_count(&name_rcc));
    drop(name_rcc);
    // Weak -> Rc
    println!("{:?}", name_weak.upgrade());

    // LinkedList
    let mut ll = LinkedList::new().append(1).append(2);
    // ll.append(1);
    // ll.append(2);
    println!("{:?}", ll);
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<LinkedListNode<T>>>,
}
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct LinkedListNode<T> {
    data: T,
    next: Option<Rc<LinkedListNode<T>>>,
    prev: Option<Weak<LinkedListNode<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    // fn append(&self, data: T) -> Self {
    //     LinkedList {
    //         head: Some(Rc::new(Node {
    //             data: data,
    //             next: self.head.clone(),
    //         })),
    //     }
    //     // let node = Rc::new(Node {
    //     //     data: data,
    //     //     next: self.head.clone(),
    //     // });
    //     // self.head = Some(node);
    //     // self
    // }

    fn append(&mut self, data: T) -> Self {
        let new_node = Rc::new(LinkedListNode {
            data: data,
            next: self.head.clone(),
            prev: None,
        });
        match self.head.clone() {
            Some(node) => {
                node.prev = Some(Rc::downgrade(&new_node));
            }
            None => {}
        }
        LinkedList {
            head: Some(new_node),
        }
    }
}
