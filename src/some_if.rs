#[macro_export]
macro_rules! some_if {
    ($res:expr, $cond:expr) => {
        if $cond {
            Some($res)
        } else {
            None
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn some_if1() {
        let res = some_if!("even", 8 % 2 == 0);

        assert_eq!(res, Some("even"));
    }

    #[test]
    fn some_if2() {
        let res = some_if!("odd", 7 % 2 == 0);

        assert_eq!(res, None);
    }
}
