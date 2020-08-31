use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Default, Debug)]
pub struct Node<T>(Rc<NodeInternal<T>>);

#[derive(Default, Debug)]
struct NodeInternal<T> {
    data: T,
    parent: Weak<Self>,
    children: RefCell<Vec<Node<T>>>,
    // children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        let internal = NodeInternal {
            data,
            parent: Weak::new(),
            children: RefCell::new(Vec::new()),
        };

        Self(Rc::new(internal))
    }

    pub fn add_all_children(&mut self, children_data: impl IntoIterator<Item = T>) {
        let this_node = self.get_rc();
        let mut children = children_data
            .into_iter()
            .map(|c| NodeInternal {
                data: c,
                parent: Rc::downgrade(this_node),
                children: RefCell::new(Vec::new()),
            })
            .map(|i| Self(Rc::new(i)))
            .collect();

        this_node.children.borrow_mut().append(&mut children);
    }

    pub fn add_child(&mut self, mut child_data: T) {
        let internal = NodeInternal {
            data: child_data,
            parent: Rc::downgrade(self.get_rc()),
            children: RefCell::new(Vec::new()),
        };

        // let x = Rc::get_mut(self.get_rc()).expect("get_mut failed in add_child");
        self.get_rc()
            .children
            .borrow_mut()
            .push(Self(Rc::new(internal)));
    }

    pub fn data(&self) -> &T {
        &self.get_rc().data
    }

    pub fn children(&self) -> Ref<Vec<Self>> {
        let rc = self.get_rc();

        rc.children.borrow()
    }

    fn get_rc(&self) -> &Rc<NodeInternal<T>> {
        &self.0
    }

    fn get_rc_mut(&mut self) -> &mut Rc<NodeInternal<T>> {
        &mut self.0
    }

    fn get_rc_clone(&self) -> Rc<NodeInternal<T>> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_root_node() {
        let root = Node::new(42);

        assert_eq!(42, *root.data());
    }

    #[test]
    fn root_with_one_child() {
        let mut root = Node::new(42);

        root.add_child(99);

        let child_ref = &root.children()[0];

        assert_eq!(99, *child_ref.data());
    }

    #[test]
    fn root_with_many_children() {
        let mut root = Node::new(42);

        root.add_child(99);
        root.add_child(50);
        root.add_child(47);
        root.add_child(12);

        let children_ref = root.children();

        assert_eq!(99, *children_ref[0].data());
        assert_eq!(50, *children_ref[1].data());
        assert_eq!(47, *children_ref[2].data());
        assert_eq!(12, *children_ref[3].data());
    }

    #[test]
    fn cannot_update_children_when_borrowing_children() {
        let mut root = Node::new(42);

        root.add_child(50);

        // Won't build unless we limit the lifetime of the ref,
        // as expected. (thought I thought modern Rust would auto-limit the lifetime...)
        let data = {
            let children = root.children();
            *children[0].data()
        };

        root.add_child(49);

        assert_eq!(50, data);
    }
}
