use rstest::rstest;
use sdp::data_transfer_mode::DataTransferMode;

#[rstest]
#[case(0b00, Some(DataTransferMode::Inactive))]
#[case(0b01, Some(DataTransferMode::Receive))]
#[case(0b10, Some(DataTransferMode::Send))]
#[case(0b11, Some(DataTransferMode::SendReceive))]
#[case(0b100, None)]
fn test_from_bits(#[case] bits: u8, #[case] expected: Option<DataTransferMode>) {
    let result = DataTransferMode::from_bit(bits);
    assert_eq!(result, expected);
}

#[rstest]
#[case(DataTransferMode::Inactive, 0b00)]
#[case(DataTransferMode::Receive, 0b01)]
#[case(DataTransferMode::Send, 0b10)]
#[case(DataTransferMode::SendReceive, 0b11)]
fn test_bits(#[case] mode: DataTransferMode, #[case] expected: u8) {
    let result = mode.as_u8();
    assert_eq!(result, expected);
}