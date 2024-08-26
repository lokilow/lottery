use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<f32> = RefCell::new(f32::from(0.0));
}

/// Get the value of the counter.
#[ic_cdk::query]
fn get() -> f32 {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

/// Set the value of the counter.
#[ic_cdk::update]
fn set(n: f32) {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    COUNTER.with(|count| *count.borrow_mut() = n);
}

/// Increment the value of the counter.
#[ic_cdk::update]
fn inc() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let expected = f32::from(42_u16);
        set(expected.clone());
        assert_eq!(get(), expected);
    }

    #[test]
    fn test_init() {
        assert_eq!(get(), f32::from(0.0));
    }

    #[test]
    fn test_inc() {
        for i in 1..10_u16 {
            inc();
            assert_eq!(get(), f32::from(i));
        }
    }
}
