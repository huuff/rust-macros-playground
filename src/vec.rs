#[macro_export]
macro_rules! vecc {
    ($( $x:expr ), *) => {
        {
            let mut tmp = Vec::new();
            $(
              tmp.push($x);
            )*
            tmp
        }
    }
}

// #[macro_export]
// macro_rules! veccc {}

#[cfg(test)]
mod tests {
    #[test]
    fn vecs_nicely() {
        let vec = vecc![1, 2, 3, 4, 5];

        // assert_eq!(vec.capacity(), 5);
        assert_eq!(vec, &[1, 2, 3, 4, 5]);
    }
}
