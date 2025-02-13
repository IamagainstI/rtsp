use media::{codec::Codec, video::h265_codec::H265Codec};
use rstest::rstest;

#[rstest]
#[case(
    90000,
    None,
    b"a=fmtp:96 profile-id=1; sprop-vps=Z0IAH5WoFAFuQA==; sprop-sps=Z0IAH5WoFAFuQA==; sprop-pps=aM4G4g==",
    Ok(H265Codec::new(
        96,
        90000,
        None,
        1,
        vec![
            103, 66, 0, 31, 149, 168, 20, 1, 
            110, 64, 104, 206, 6, 226, 103, 
            66, 0, 31, 149, 168, 20, 1, 110, 64
        ],
    ))
)]
#[case(
    90000,
    None,
    b"a=fmtp:96 profile-id=1; sprop-vps=Z0IAH5WoFAFuQA==; sprop-sps=Z0IAH5WoFAFuQA==",
    Err(())
)]
#[case(
    90000,
    None,
    b"a=fmtp:96 profile-id=1; sprop-vps=Z0IAH5WoFAFuQA==; sprop-pps=aM4G4g==",
    Err(())
)]
#[case(
    90000,
    None,
    b"a=fmtp:96 profile-id=1; sprop-sps=Z0IAH5WoFAFuQA==; sprop-pps=aM4G4g==",
    Err(())
)]
#[case(
    90000,
    None,
    b"invalid fmtp data",
    Err(()),
)]
fn test_from_fmtp_internal(
    #[case] clock_rate: u32,
    #[case] channel_count: Option<u8>,
    #[case] data: &[u8],
    #[case] expected: Result<H265Codec, ()>,
) {
    let result = H265Codec::parse(clock_rate, channel_count, data).map_err(|_| ());
    assert_eq!(result, expected);
}