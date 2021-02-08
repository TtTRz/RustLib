use std::{rc::{Rc, Weak}};
use std::cell::RefCell;

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
  let mut list = LinkedList::new();
  // append_front
  list.append_front(1);
  list.append_front(2);
  println!("{:?}", list);
  //append_end

  // DbLinkedList
  let mut dblist = DbLinkedList::new();
  dblist.append(1);
  dblist.append(2);
  println!("{:?}", dblist);

}

#[derive(Debug)]
struct LinkedList<T> {
  head: Option<Rc<Node<T>>>
}
#[derive(Debug)]
pub struct Node<T> {
  data: T,
  next: Option<Rc<Node<T>>>,
}
impl <T> LinkedList<T> {
  fn new() -> Self {
    LinkedList {
      head: None,
    }
  }
  fn append_front(&mut self, data: T) {
    let front_node = Rc::new(Node {
      data,
      next: self.head.take(),
    });
    self.head = Some(front_node.clone())
  }
  // TODO push_end
}

#[derive(Debug)]
struct DbLinkedList<T> {
  head: Option<Rc<RefCell<DbNode<T>>>>,
}
#[derive(Debug)]
struct DbNode<T> {
  data: T,
  next: Option<Rc<RefCell<DbNode<T>>>>,
  prev: Option<Weak<RefCell<DbNode<T>>>>
}

impl<T> DbLinkedList<T> {
  fn new() -> Self{
    DbLinkedList {
      head: None,
    }
  }
  
  fn append(&mut self, data: T) {
    match self.head.take() {
      Some(head_node) => {
        let node = Rc::new(RefCell::new(DbNode {
          data,
          next: Some(head_node.clone()),
          prev: None,
        }));
        let mut m = head_node.borrow_mut();
        m.prev = Some(Rc::downgrade(&node));
        self.head = Some(node)
      },
      None => {
        self.head = Some(Rc::new(RefCell::new(DbNode {
          data,
          next: None,
          prev: None,
        })))
      },
    }
  }
}
