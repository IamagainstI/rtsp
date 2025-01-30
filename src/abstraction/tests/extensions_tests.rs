use abstractions::extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt};
use rstest::rstest;

#[rstest]
#[case(b"Hello, world!", true)]
#[case(&[0xff, 0xfe, 0xfd], false)]
fn test_is_utf8(#[case] input: &[u8], #[case] expected: bool) {
    assert_eq!(input.is_utf8(), expected);
}

#[rstest]
#[case(b"Hello, world!", Ok("Hello, world!"))]
#[case(&[0xff, 0xfe, 0xfd], Err(()))]
fn test_utf8_to_str(#[case] input: &[u8], #[case] expected: Result<&str, ()>) {
    let result = input.utf8_to_str().map_err(|_| ());
    assert_eq!(result, expected);
}


#[rstest]
#[case(b"12345", Ok(12345))]
#[case(b"-67890", Ok(-67890))]
#[case(b"12a45", Err(()))]
fn test_utf8_to_integer(#[case] input: &[u8], #[case] expected: Result<i32, ()>) {
    let result: Result<i32, ()> = input.utf8_to_number::<i32>().map_err(|_| ());
    assert_eq!(result, expected);
}

#[rstest]
#[case(vec![0, 0, 1, 2, 3, 0, 4, 5, 0, 0], 0, vec![1, 2, 3, 0, 4, 5])]
#[case(vec![1, 2, 3], 0, vec![1, 2, 3])]
#[case(vec![0, 0, 0], 0, vec![])]
fn test_trim(#[case] input: Vec<i32>, #[case] trim_value: i32, #[case] expected: Vec<i32>) {
    assert_eq!(input.trim(&trim_value), expected.as_slice());
}

#[rstest]
#[case(vec![1, 2, 3, 4, 5], 3, Some((&[1, 2][..], &[4, 5][..])))]
#[case(vec![1, 2, 3, 4, 5], 6, None)]
fn test_separate(#[case] input: Vec<i32>, #[case] separator: i32, #[case] expected: Option<(&[i32], &[i32])>) {
    assert_eq!(input.separate(separator), expected);
}

#[rstest]
#[case(vec![0, 1, 2, 3, 0, 4, 5, 0], 3, 0, Some((&[1, 2][..], &[4, 5][..])))]
#[case(vec![0, 1, 2, 3, 0, 4, 5, 0], 6, 0, None)]
fn test_separate_trimmed(#[case] input: Vec<i32>, #[case] separator: i32, #[case] trim_value: i32, #[case] expected: Option<(&[i32], &[i32])>) {
    assert_eq!(input.separate_trimmed(separator, &trim_value), expected);
}