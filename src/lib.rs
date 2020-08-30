mod node;
mod write_once_lock;
mod merger;

struct MctsData {
    rollouts: usize,
    wins: usize,
    is_saturated: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
