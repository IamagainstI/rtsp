use media::{audio::aac_codec::AacCodec, codec::Codec};
use rstest::rstest;

#[rstest]
#[case(
    48000,
    Some(2),
    b"a=fmtp:97 streamtype=5; profile-level-id=15; mode=AAC-hbr; config=1190; sizeLength=13; indexLength=3; indexDeltaLength=3; profile=1;\r\n",
    Ok(AacCodec::new(
        97,
        48000,
        Some(2),
        13,
        3,
        3,
        Some(vec![0x11, 0x90]),
    ))
)]
#[case(
    48000,
    Some(2),
    b"sizeLength=13; indexLength=3; indexDeltaLength=3; config=invalid",
    Err(())
)]
#[case(
    48000,
    Some(2),
    b"invalid fmtp data",
    Err(())
)]
fn test_from_fmtp_internal(
    #[case] clock_rate: u32,
    #[case] channel_count: Option<u8>,
    #[case] data: &[u8],
    #[case] expected: Result<AacCodec, ()>,
) 
{
    let result = AacCodec::parse(clock_rate, channel_count, data).map_err(|_| ());
    assert_eq!(result, expected);
}