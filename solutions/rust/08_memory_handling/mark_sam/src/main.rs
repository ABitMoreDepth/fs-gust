use std::cell::RefCell;

#[macro_use]
extern crate derive_new;

struct Arena<T> {
    nodes: RefCell<Vec<T>>,
}

impl<T> Arena<T> {
    fn new(size: usize) -> Arena<T> {
        Arena {
            nodes: RefCell::new(Vec::<T>::with_capacity(size)),
        }
    }

    fn allocate(&self, node: T) -> &T {
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(node);
        let value_ptr: *const T = nodes.last().unwrap();
        unsafe { &*value_ptr }
    }
}

#[derive(new)]
struct Node<'a> {
    name: RefCell<String>,

    #[new(default)]
    children: RefCell<Vec<&'a Node<'a>>>,
}

fn main() {
    let arena = Arena::new(3);

    let parent = arena.allocate(Node::new(RefCell::new("Parent".to_string())));
    let child1 = arena.allocate(Node::new(RefCell::new("C1".to_string())));
    let child2 = arena.allocate(Node::new(RefCell::new("C2".to_string())));

    child2.children.borrow_mut().push(parent);
    parent.children.borrow_mut().push(child1);
    parent.children.borrow_mut().push(child2);

    let a = *parent.children.borrow().get(0).unwrap();
    let b = *parent.children.borrow().get(1).unwrap();

    do_stuff_to_children(a, b);
}

fn do_stuff_to_children(c1: &Node, c2: &Node) {
    c1.name.replace(c2.name.borrow().clone());
    c2.name.replace(c1.name.borrow().clone());
}
