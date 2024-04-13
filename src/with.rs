#[macro_export]
macro_rules! with {
    ($this:ident, $($prop:ident = $val:expr);*) => {
        $($this.$prop = $val);*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn withs() {
        #[derive(Default)]
        struct TestStruct {
            pub a: usize,
            pub b: usize,
            pub c: usize,
        }

        let mut this = TestStruct::default();

        with!(this,
            a = 1;
            b = 2;
            c = 3);

        assert_eq!(&this.a, &1);
        assert_eq!(&this.b, &2);
        assert_eq!(&this.c, &3);
    }
}
