use std::collections::VecDeque;

fn main() {
    let mut v: VecDeque<i32> = VecDeque::new();
    v.push_back(1);
    assert_eq!(*v.get(0).unwrap(), 1);

    v.push_back(2);
    v.push_back(3);
    assert_eq!(*v.get(0).unwrap(), 1);
    assert_eq!(*v.get(1).unwrap(), 2);
    assert_eq!(*v.get(2).unwrap(), 3);

    v.pop_back();
    assert_eq!(v.get(3), None);

    v.push_front(4);
    assert_eq!(*v.get(0).unwrap(), 4);
    assert_eq!(*v.get(1).unwrap(), 1);
    assert_eq!(*v.get(2).unwrap(), 2);

    v.pop_front();
    assert_eq!(*v.get(0).unwrap(), 1);
    assert_eq!(*v.get(1).unwrap(), 2);
    assert_eq!(v.get(2), None);

    let len = v.len();
    assert_eq!(len, 2);

    let is_empty = v.is_empty();
    assert_eq!(is_empty, false);
}
