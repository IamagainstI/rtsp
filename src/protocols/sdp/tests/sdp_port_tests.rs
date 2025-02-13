use sdp::sdp_port::SdpPort;

#[test]
fn test_impl() {
    let rtp_port = 49170;
    let rtcp_port = 49171;
    let sdp_port = SdpPort::new(rtp_port, rtcp_port);
    assert_eq!(sdp_port.rtp_port(), rtp_port);
    assert_eq!(sdp_port.rtcp_port(), rtcp_port);
}