//selection sort
#[cfg(test)]
mod tests {
    #[test]
    fn sort_numbers() {
        use crate::sort_algo::selection_sort;

        let mut res = Vec::<u8>::new();
        selection_sort(&mut res);
        assert_eq!(res, vec![]);

        let mut res = vec!["a"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a"]);

        let mut res = vec!["a", "b", "c"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c"]);

        let mut res = vec!["d", "a", "c", "b"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }
}