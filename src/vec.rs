// TODO: Disable clippy lint
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

/// Like vecc but also creates a vector of the right capacity
#[macro_export]
macro_rules! veccc {
    ($( $x:expr), *) => {
        <[_]>::into_vec(Box::new([$($x),*]))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn vecs_nicely() {
        let vec = vecc![1, 2, 3, 4, 5];

        // assert_eq!(vec.capacity(), 5);
        assert_eq!(vec, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn vecs_nicely_and_has_right_capacity() {
        let vec = veccc![1, 2, 3, 4, 5];

        assert_eq!(vec, &[1, 2, 3, 4, 5]);
        assert_eq!(vec.capacity(), 5);
    }
}
