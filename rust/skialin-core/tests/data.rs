use skialin_core::Data;

#[test]
fn with_copy_roundtrips_bytes() {
    let data = Data::with_copy(b"hello world");
    assert_eq!(data.size(), 11);
    assert!(!data.is_empty());
    assert_eq!(data.as_bytes(), b"hello world");
}

#[test]
fn empty_is_empty() {
    let data = Data::empty();
    assert_eq!(data.size(), 0);
    assert!(data.is_empty());
}

#[test]
fn uninitialized_then_write() {
    let mut data = Data::uninitialized(5);
    assert_eq!(data.size(), 5);
    data.writable_bytes().copy_from_slice(b"abcde");
    assert_eq!(data.as_bytes(), b"abcde");
}

#[test]
fn zero_initialized_is_all_zero() {
    let data = Data::zero_initialized(4);
    assert_eq!(data.as_bytes(), &[0, 0, 0, 0]);
}

#[test]
fn copy_range_extracts_middle() {
    let data = Data::with_copy(b"0123456789");
    assert_eq!(data.copy_range(2, 3), b"234");
}

#[test]
fn copy_subset_and_share_subset() {
    let data = Data::with_copy(b"0123456789");

    let copy = data.copy_subset(3, 4).unwrap();
    assert_eq!(copy.as_bytes(), b"3456");

    let shared = data.share_subset(3, 4).unwrap();
    assert_eq!(shared.as_bytes(), b"3456");

    assert!(data.copy_subset(8, 10).is_none());
}

#[test]
fn equals_compares_contents() {
    let a = Data::with_copy(b"same");
    let b = Data::with_copy(b"same");
    let c = Data::with_copy(b"different");
    assert!(a.equals(&b));
    assert!(!a.equals(&c));
}

#[test]
fn from_missing_file_is_none() {
    assert!(Data::from_file("Z:/definitely/does/not/exist.bin").is_none());
}
