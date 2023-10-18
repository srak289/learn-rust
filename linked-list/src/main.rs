use std::ptr;

#[derive(Debug)]
struct Node<T> {
    data: Box<T>,
    p_node: *mut Node<T>,
    n_node: *mut Node<T>,
}

#[derive(Debug)]
struct List<T> {
    node: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(d: Box<T>) -> *mut Node<T> {
        &mut Self {
            data: d,
            p_node: ptr::null_mut(),
            n_node: ptr::null_mut(),
        }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            node: ptr::null_mut(),
        }
    }

    fn add_node(d: Box<T>) {
        
    }

    fn current_node(&self) -> *mut Node<T> {
        self.node
    }
}

fn main() {
    let mut list = List::<i32>::new();
    list.add_node(3);
    println!("{:?}", list);
}
