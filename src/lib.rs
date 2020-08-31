mod merger;
mod node;

#[derive(Default, Debug)]
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
