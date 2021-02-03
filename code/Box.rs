#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug, Default)]
struct Tree {
    root: i64,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn new(root: i64) -> Tree {
        Tree {
            root: root,
            ..Default::default()
        }
    }

    fn left(mut self, leaf: Tree) -> Self {
        self.left = Some(Box::new(leaf));
        self
    }

    fn right(mut self, leaf: Tree) -> Self {
        self.right = Some(Box::new(leaf));
        self
    }

    fn set_left(&mut self, leaf: Tree) -> &mut Self {
        self.left = Some(Box::new(leaf));
        self
    }

    fn set_right(&mut self, leaf: Tree) -> &mut Self {
        self.right = Some(Box::new(leaf));
        self
    }
}

fn main() {
    let a = Box::new(1);
    println!("value of a is : {}", a);
    let b = a;
    // println!("value of a is : {}", a);  move to b
    println!("value of b is : {}", b);

    use List::{Cons, Nil};
    let list = List::Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    // 编译失败
    // let t = Tree {
    //   root: 0,
    //   left: Some(Tree {
    //     root: 1,
    //     left: None,
    //     right: None
    //   }),
    //   right: None
    // };

    // use Box
    let t = Tree {
        root: 0,
        left: Some(Box::new(Tree {
            root: 1,
            left: None,
            right: None,
        })),
        right: None,
    };
    println!("{:?}", t);

    let mut default_t = Tree::new(0);
    println!("default Tree: {:?}", default_t);

    default_t = default_t
        .left(Tree::new(1))
        .right(Tree::new(2).left(Tree::new(3)));
    println!("default Tree with left & right leaf: {:?}", default_t);

    let mut set_t = Tree::new(0);

    set_t.set_left(Tree::new(1));
    set_t.set_right(Tree::new(2).left(Tree::new(3)));

    println!("default Tree with left & right leaf: {:?}", set_t);
}
