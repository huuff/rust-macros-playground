#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[macro_export]
macro_rules! count1 {
    ($($tts:tt)*) => { 0usize $(+ replace_expr!($tts 1usize))*};
}

#[cfg(test)]
mod tests {

    #[test]
    fn counts_ok() {
        assert_eq!(5, count1![a b c d e]);
    }
}
