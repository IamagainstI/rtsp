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
#[case(vec![0, 0, 1, 2, 3, 0, 4, 5, 0, 0], vec![0], vec![1, 2, 3, 0, 4, 5])]
#[case(vec![1, 2, 3], vec![0], vec![1, 2, 3])]
#[case(vec![0, 0, 0], vec![0], vec![])]
#[case(vec![], vec![0], vec![])]
fn test_trim(#[case] input: Vec<i32>, #[case] trim_value: Vec<i32>, #[case] expected: Vec<i32>) {
    assert_eq!(input.trim(&trim_value), expected.as_slice());
}

#[rstest]
#[case(vec![1, 2, 3, 4, 5], vec![3], Some((&[1, 2][..], &[4, 5][..])))]
#[case(vec![1, 2, 3, 4, 5], vec![3, 4], Some((&[1, 2][..], &[5][..])))]
#[case(vec![1, 2, 3, 4, 5], vec![1, 2], Some((&[][..], &[3, 4, 5][..])))]
#[case(vec![1, 2, 3, 4, 5], vec![4, 5], Some((&[1, 2, 3][..], &[][..])))]
#[case(vec![1, 2, 3], vec![2], Some((&[1][..], &[3][..])))]
#[case(vec![1, 2, 3, 4, 5], vec![6], None)]
#[case(vec![1, 2, 3, 4, 5], vec![2, 4], None)]
fn test_separate(#[case] input: Vec<i32>, #[case] separators: Vec<i32>, #[case] expected: Option<(&[i32], &[i32])>) {
    assert_eq!(input.separate(&separators), expected);
}

#[rstest]
#[case(vec![0, 1, 2, 3, 0, 4, 5, 0], vec![3], vec![0], Some((&[1, 2][..], &[4, 5][..])))]
#[case(vec![1, 2, 3], vec![2], vec![0], Some((&[1][..], &[3][..])))]
#[case(vec![0, 1, 2, 3, 0, 4, 5, 0], vec![6], vec![0], None)]
#[case(vec![0, 1, 2, 3, 0, 4, 5, 0], vec![2, 4], vec![0], None)]
fn test_separate_trimmed(#[case] input: Vec<i32>, #[case] separators: Vec<i32>, #[case] trim_value: Vec<i32>, #[case] expected: Option<(&[i32], &[i32])>) {
    assert_eq!(input.separate_trimmed(&separators, &trim_value), expected);
}

#[rstest]
#[case(&[1, 2, 3, 4, 5], &[3], Some((&[1, 2][..], &[4, 5][..])))]
#[case(&[1, 2, 3, 4, 5], &[6], Some((&[1, 2, 3, 4, 5][..], &[][..])))]
#[case(&[1, 2, 3, 4, 5], &[1], Some((&[][..], &[2, 3, 4, 5][..])))]
#[case(&[1, 2, 3, 4, 5], &[5], Some((&[1, 2, 3, 4][..], &[][..])))]
#[case(&[1, 2, 3, 4, 5], &[0], Some((&[1, 2, 3, 4, 5][..], &[][..])))]
fn test_while_separate(#[case] slice: &[i32], #[case] elems: &[i32], #[case] expected: Option<(&[i32], &[i32])>) {
    let result = slice.while_separate(elems);
    assert_eq!(result, expected);
}

#[rstest]
#[case(&[1, 2, 3, 4, 5], &[3], &[1, 5], Some((&[2][..], &[4][..])))]
#[case(&[1, 2, 3, 4, 5], &[6], &[1, 5], Some((&[2, 3, 4][..], &[][..])))]
#[case(&[1, 2, 3, 4, 5], &[1], &[1, 5], Some((&[][..], &[2, 3, 4][..])))]
#[case(&[1, 2, 3, 4, 5], &[5], &[1, 5], Some((&[2, 3, 4][..], &[][..])))]
#[case(&[1, 2, 3, 4, 5], &[0], &[1, 5], Some((&[2, 3, 4][..], &[][..])))]
fn test_while_separate_trimmed(#[case] slice: &[i32], #[case] elems: &[i32], #[case] trim: &[i32], #[case] expected: Option<(&[i32], &[i32])>) {
    let result = slice.while_separate_trimmed(elems, trim);
    assert_eq!(result, expected);
}