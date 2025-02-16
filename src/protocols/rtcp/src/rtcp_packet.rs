use crate::{application_defined::ApplicationDefined, goodbye::Goodbye, receiver_report::ReceiverReport, sender_report::SenderReport, source_description::SourceDescription};

/// Represents an RTCP packet.
#[derive(Debug)]
pub enum RtcpPacket<'a> {
    SenderReport(SenderReport<'a>),
    ReceiverReport(ReceiverReport<'a>),
    SourceDescription(SourceDescription<'a>),
    Goodbye(Goodbye<'a>),
    ApplicationDefined(ApplicationDefined<'a>),
}