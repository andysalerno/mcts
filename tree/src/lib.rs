mod merger;
pub mod node;

#[cfg(test)]
mod tests {
    use super::*;
    use node::Node;

    #[test]
    fn copy_type_works() {
        let x = Node::new(48);
    }
}
