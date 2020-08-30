use std::sync::{Arc, Weak};

struct Node<T> {
    data: T,
    parent: Weak<Self>,
}
