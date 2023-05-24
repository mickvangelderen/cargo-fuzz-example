pub fn do_a_thing(input: &[u8]) -> Option<usize> {
    let [p, o, w] = [
        *input.get(0)?,
        *input.get(10)?,
        *input.get(20)?,
    ];

    if (p, o, w) == (b'p', b'o', b'w') {
        panic!("We don't accept pow at these indices!")
    }

    Some(input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_test_passes_then_the_implementation_must_be_fine() {
        let result = do_a_thing("Hello, world! Here is some text.".as_bytes());
        assert_eq!(result, Some(32));
    }
}
