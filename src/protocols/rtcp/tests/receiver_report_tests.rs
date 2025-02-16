use rtcp::rtcp_packet_type::RtcpPacketType;
use rtcp::receiver_report::ReceiverReport;
use rtcp::report_block::ReportBlock;

#[test]
fn test_receiver_report_try_from() {
    let buffer = [
        0b10000001, 201, 0x00, 0x07, // RTCP header
        0x12, 0x34, 0x56, 0x78, // SSRC
        // Report block
        0x12, 0x34, 0x56, 0x78, // SSRC
        0x01, // Fraction lost
        0x00, 0x00, 0x02, // Cumulative lost
        0x00, 0x00, 0x00, 0x03, // Highest sequence number
        0x00, 0x00, 0x00, 0x04, // Jitter
        0x00, 0x00, 0x00, 0x05, // Last SR
        0x00, 0x00, 0x00, 0x06, // Delay since last SR
    ];

    let receiver_report = ReceiverReport::try_from(&buffer[..]).unwrap();

    assert_eq!(receiver_report.header().version(), 2);
    assert_eq!(receiver_report.header().padding(), false);
    assert_eq!(receiver_report.header().report_count(), 1);
    assert_eq!(receiver_report.header().packet_type(), &RtcpPacketType::ReceiverReport);
    assert_eq!(receiver_report.header().length(), 7);
    assert_eq!(receiver_report.ssrc(), 0x12345678);

    let report_blocks = receiver_report.report_blocks();
    assert_eq!(report_blocks.len(), 1);
    assert_eq!(report_blocks[0].ssrc(), 0x12345678);
    assert_eq!(report_blocks[0].fraction_lost(), 0x01);
    assert_eq!(report_blocks[0].cumulative_lost(), 0x000002);
    assert_eq!(report_blocks[0].highest_seq_num(), 0x00000003);
    assert_eq!(report_blocks[0].jitter(), 0x00000004);
    assert_eq!(report_blocks[0].last_sr(), 0x00000005);
    assert_eq!(report_blocks[0].delay_since_last_sr(), 0x00000006);
}

#[test]
fn test_receiver_report_try_from_short_buffer() {
    let buffer = [
        0b10000001, 201, 0x00, 0x07, // RTCP header
        0x12, 0x34, 0x56, 0x78, // SSRC
        // Missing report blocks
    ];

    let result = ReceiverReport::try_from(&buffer[..]);
    assert!(result.is_err());
}

#[test]
fn test_report_block_try_from() {
    let buffer = [
        0x12, 0x34, 0x56, 0x78, // SSRC
        0x01, // Fraction lost
        0x00, 0x00, 0x02, // Cumulative lost
        0x00, 0x00, 0x00, 0x03, // Highest sequence number
        0x00, 0x00, 0x00, 0x04, // Jitter
        0x00, 0x00, 0x00, 0x05, // Last SR
        0x00, 0x00, 0x00, 0x06, // Delay since last SR
    ];

    let report_block = ReportBlock::try_from(&buffer[..]).unwrap();

    assert_eq!(report_block.ssrc(), 0x12345678);
    assert_eq!(report_block.fraction_lost(), 0x01);
    assert_eq!(report_block.cumulative_lost(), 0x000002);
    assert_eq!(report_block.highest_seq_num(), 0x00000003);
    assert_eq!(report_block.jitter(), 0x00000004);
    assert_eq!(report_block.last_sr(), 0x00000005);
    assert_eq!(report_block.delay_since_last_sr(), 0x00000006);
}

#[test]
fn test_report_block_try_from_short_buffer() {
    let buffer = [
        0x12, 0x34, 0x56, 0x78, // SSRC
        0x01, // Fraction lost
        0x00, 0x00, 0x02, // Cumulative lost
        0x00, 0x00, 0x00, 0x03, // Highest sequence number
        0x00, 0x00, 0x00, 0x04, // Jitter
        0x00, 0x00, 0x00, // Incomplete Last SR
    ];

    let result = ReportBlock::try_from(&buffer[..]);
    assert!(result.is_err());
}