#[cfg(test)]
mod tests {
    use paretome_rust::pareto::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
