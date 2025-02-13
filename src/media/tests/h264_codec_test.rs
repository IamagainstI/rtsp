use media::{codec::Codec, video::h264_codec::{H264Codec, PackatizationMode}};
use rstest::rstest;

#[rstest]
#[case(
    90000,
    None,
    b"a=fmtp:96 profile-level-id=42e01f; packetization-mode=1; sprop-parameter-sets=Z0IAH5WoFAFuQA==,aM4G4g==",
    Ok(H264Codec::new(
        96,
        90000,
        None,
        "42e01f".to_string(),
        PackatizationMode::NonInterleaved,
        vec![
            0, 0, 0, 1, 103, 66, 0, 31, 149, 168, 20, 1, 110, 64, 
            0, 0, 0, 1, 103, 66, 0, 31, 149, 168, 20, 1, 110, 64
        ],
    ))
)]
#[case(
    90000,
    None,
    b"a=fmtp:96 profile-level-id=42e01f; packetization-mode=0; sprop-parameter-sets=Z0IAH5WoFAFuQA==",
    Ok(H264Codec::new(
        96,
        90000,
        None,
        "42e01f".to_string(),
        PackatizationMode::SingleNalUnit,
        vec![
            0, 0, 0, 1, 103, 66, 0, 31, 149, 168, 20, 1, 110, 64
        ],
    ))
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
    #[case] expected: Result<H264Codec, ()>,
) {
    let result = H264Codec::parse(clock_rate, channel_count, data).map_err(|_| ());
    assert_eq!(result, expected);
}