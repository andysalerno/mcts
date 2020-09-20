use std::cell::{Ref, RefCell, RefMut};
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

    pub fn clone_rc(&self) -> Self {
        Self(self.get_rc_clone())
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

    pub fn data(&self) -> &T {
        &self.get_rc().data
    }

    pub fn children(&self) -> Ref<Vec<Self>> {
        let rc = self.get_rc();

        rc.children.borrow()
    }

    pub fn children_mut(&mut self) -> RefMut<Vec<Self>> {
        let rc = self.get_rc();

        rc.children.borrow_mut()
    }

    pub fn parent(&self) -> Option<Self> {
        let maybe_rc = self.get_rc().parent.upgrade();

        maybe_rc.map(|rc| Self(rc))
    }

    fn get_rc(&self) -> &Rc<NodeInternal<T>> {
        &self.0
    }

    /// Consider removing this -- if we can expose the data
    /// and the children as mut already, no need to expose the whole
    /// Rc, you can just pick which you need.
    fn get_rc_mut(&mut self) -> &mut Rc<NodeInternal<T>> {
        &mut self.0
    }

    fn get_rc_clone(&self) -> Rc<NodeInternal<T>> {
        self.0.clone()
    }

    fn add_child(&mut self, child_data: T) {
        let internal = NodeInternal {
            data: child_data,
            parent: Rc::downgrade(self.get_rc()),
            children: RefCell::new(Vec::new()),
        };

        self.get_rc()
            .children
            .borrow_mut()
            .push(Self(Rc::new(internal)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An i32 as a struct, to prevent it from implementing Copy,
    /// which would be unrealistic for actual scenarios.
    #[derive(Debug, PartialEq)]
    struct NoCopy(i32);

    #[test]
    fn new_root_node() {
        let root = Node::new(NoCopy(42));

        assert_eq!(NoCopy(42), *root.data());
    }

    #[test]
    fn root_with_one_child() {
        let mut root = Node::new(NoCopy(42));

        root.add_child(NoCopy(99));

        let child_ref = &root.children()[0];

        assert_eq!(NoCopy(99), *child_ref.data());
    }

    #[test]
    fn root_with_many_children() {
        let mut root = Node::new(NoCopy(42));

        root.add_child(NoCopy(99));
        root.add_child(NoCopy(50));
        root.add_child(NoCopy(47));
        root.add_child(NoCopy(12));

        let children_ref = root.children();

        assert_eq!(NoCopy(99), *children_ref[0].data());
        assert_eq!(NoCopy(50), *children_ref[1].data());
        assert_eq!(NoCopy(47), *children_ref[2].data());
        assert_eq!(NoCopy(12), *children_ref[3].data());
    }

    #[test]
    fn cannot_update_children_when_borrowing_children() {
        let mut root = Node::new(NoCopy(42));

        root.add_child(NoCopy(50));

        let children = root.children();
        assert_eq!(1, children.len());
        let data = children[0].data();

        // Won't build -- cannot mutate while already borrowed.
        // root.add_child(NoCopy(49));

        assert_eq!(&NoCopy(50), data);
        assert_eq!(1, root.children().len());
    }

    #[test]
    fn multiple_layers_of_children() {
        let mut root = Node::new(NoCopy(42));

        root.add_child(NoCopy(1));
        root.add_child(NoCopy(2));

        // Should not panic
        {
            let added_child = &mut root.children_mut()[0];
            added_child.add_all_children(vec![NoCopy(3), NoCopy(4), NoCopy(5)]);
        }

        {
            let added_child2 = &mut root.children_mut()[1];
            added_child2.add_all_children(vec![NoCopy(6), NoCopy(7), NoCopy(8), NoCopy(9)]);
        }

        assert_eq!(2, root.children().len());
        assert_eq!(3, root.children()[0].children().len());
        assert_eq!(4, root.children()[1].children().len());
    }

    #[test]
    fn parent_when_node_has_parent() {
        let mut root = Node::new(NoCopy(42));

        root.add_child(NoCopy(1));

        let child = &root.children()[0];

        let parent = child
            .parent()
            .expect("In this test, the child *does* have a parent.");

        assert_eq!(NoCopy(42), *parent.data());
    }

    #[test]
    fn parent_when_node_is_root() {
        let mut root = Node::new(NoCopy(42));

        root.add_child(NoCopy(1));

        let _child = &root.children()[0];

        let parent = root.parent();

        assert!(parent.is_none());
    }

    #[test]
    fn complex_walk_operations() {
        let mut v = 0;
        let mut s = 0;
        let sum = &mut s;
        let mut i = move || {
            v += 1;
            *sum += v;
            v
        };

        let mut root = Node::new(i());

        root.add_all_children(vec![i(), i(), i()]);

        for child in root.children_mut().iter_mut() {
            child.add_all_children(vec![i(), i(), i(), i()]);
        }

        let mut stack = Vec::new();
        stack.push(root.clone_rc());

        let mut test_sum = 0;
        while let Some(r) = stack.pop() {
            test_sum += r.data();
            r.children().iter().for_each(|c| stack.push(c.clone_rc()));
        }

        assert_eq!(s, test_sum);
    }
}
