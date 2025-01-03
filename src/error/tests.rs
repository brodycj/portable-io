#[cfg(feature = "alloc")]
use core::{error,fmt};
use core::mem::size_of;

extern crate alloc;
use alloc::format;
use alloc::string::ToString;

use super::{Error, ErrorKind};

#[test]
fn test_size() {
    assert!(size_of::<Error>() <= size_of::<[usize; 2]>());
}

// TODO ADD & TEST MISSING FUNCTIONALITY: DEBUG ERROR - OS ERROR

#[cfg(feature = "alloc")]
#[test]
fn test_downcasting() {
    #[derive(Debug)]
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("asdf")
        }
    }

    impl error::Error for TestError {}

    // we have to call all of these UFCS style right now since method
    // resolution won't implicitly drop the Send+Sync bounds
    let mut err = Error::new(ErrorKind::Other, TestError);
    assert!(err.get_ref().unwrap().is::<TestError>());
    assert_eq!("asdf", err.get_ref().unwrap().to_string());
    assert!(err.get_mut().unwrap().is::<TestError>());
    let extracted = err.into_inner().unwrap();
    extracted.downcast::<TestError>().unwrap();
}

#[test]
fn test_const() {
    const E: Error = Error::new_const(ErrorKind::NotFound, &"hello");

    assert_eq!(E.kind(), ErrorKind::NotFound);
    assert_eq!(E.to_string(), "hello");
    assert!(format!("{:?}", E).contains("\"hello\""));
    assert!(format!("{:?}", E).contains("NotFound"));
}
