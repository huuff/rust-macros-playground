#[macro_export]
macro_rules! is_any_of {
    ($e:expr, [ $($choice:expr ),+]) => {
        $($e == $choice)||*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_any_of1() {
        // TODO: Remove clippy lint
        assert!(is_any_of!(3, [1, 2, 1 + 2]));
    }

    #[test]
    fn is_any_of2() {
        assert!(!is_any_of!(5, [1, 2, 3, 4]));
    }
}
