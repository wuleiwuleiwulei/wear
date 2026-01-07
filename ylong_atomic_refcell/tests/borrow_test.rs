use std::sync::Arc;
use std::thread::spawn;
use ylong_atomic_refcell::{AtomicRefCell, BorrowError};

#[test]
fn test_borrow() {
    let data = AtomicRefCell::new(String::from("test"));
    let _borrow1 = data.borrow();
    let _borrow2 = data.borrow();
    let _borrow3 = data.borrow();
}

#[test]
fn test_borrow_mut() {
    let data = AtomicRefCell::new(String::from("test"));
    let _borrow1 = data.borrow_mut();
    let _borrow2 = data.try_borrow_mut();
    assert_eq!(_borrow2, Err(BorrowError::AlreadyBorrowMut));
}

#[test]
fn borrow_mut_then_borrow() {
    let data = AtomicRefCell::new(String::from("test"));
    let _borrow1 = data.borrow_mut();
    let _borrow2 = data.try_borrow();
    assert_eq!(_borrow2, Err(BorrowError::AlreadyBorrowMut));
}

#[test]
fn borrow_then_borrow_mut() {
    let data = AtomicRefCell::new(String::from("test"));
    let _borrow1 = data.borrow();
    let _borrow2 = data.try_borrow_mut();
    assert_eq!(_borrow2, Err(BorrowError::AlreadyBorrow));
}

#[test]
fn cross_test() {
    let data = AtomicRefCell::new(String::from("test"));
    {
        let _borrow1 = data.borrow();
    }
    {
        let _borrow1 = data.borrow();
        let _borrow2 = data.try_borrow_mut();
        assert_eq!(_borrow2, Err(BorrowError::AlreadyBorrow));
        let _borrow3 = data.borrow();
    }
    {
        let _borrow1 = data.borrow();
        let _borrow2 = data.try_borrow_mut();
        assert_eq!(_borrow2, Err(BorrowError::AlreadyBorrow));
        drop(_borrow1);
        let _borrow3 = data.borrow_mut();
    }
    {
        let _borrow1 = data.borrow_mut();
        let _borrow2 = data.try_borrow_mut();
        assert_eq!(_borrow2, Err(BorrowError::AlreadyBorrowMut));
        drop(_borrow1);
        let _borrow3 = data.borrow_mut();
    }
}

static LOOP_TIMES: u32 = 1000;
static LOOP_THREADS: u32 = 1000;

#[test]
fn thread_borrow_test() {
    let data = Arc::new(AtomicRefCell::new(String::from("test")));
    let _ = data.borrow_mut();
    let _ = data.borrow_mut();
    let mut threads = Vec::new();
    for _ in 0..LOOP_THREADS {
        let data = data.clone();
        let handle = spawn(move || {
            for _ in 0..LOOP_TIMES {
                data.borrow();
            }
        });
        threads.push(handle);
    }
    let _ = data.borrow();
    for thread in threads {
        let _ = thread.join();
    }
    let _ = data.borrow_mut();
}
