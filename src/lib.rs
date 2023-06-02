use std::sync::Mutex;

/// Panics if the 1st, 11th and 21st bytes equal 'p', 'o' and 'w' in that order.
pub fn can_panic(input: &[u8]) -> Option<usize> {
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

/// Locks two mutexes in differing order on two separate threads, potentially causing a deadlock.
pub fn can_deadlock() {
    fn lock_yield_lock<T>(m0: &Mutex<T>, m1: &Mutex<T>) {
        let _0 = m0.lock().unwrap();
        std::thread::yield_now();
        let _1 = m1.lock().unwrap();
    }

    let a = Mutex::new(0);
    let b = Mutex::new(0);

    std::thread::scope(|scope| {
        scope.spawn(|| lock_yield_lock(&a, &b));
        scope.spawn(|| lock_yield_lock(&b, &a));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_test_passes_then_the_implementation_must_be_fine() {
        let result = can_panic("Hello, world! Here is some text.".as_bytes());
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_can_deadlock() {
        // NOTE(mickvangelderen): This number of iterations lead to the occasional hang on my machine.
        for _ in 0..200 {
            can_deadlock();
        }
        panic!("this test will either never run to completion or fail");
    }
}
