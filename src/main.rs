use memmap::Mmap;
use nscl_evt::{NsclData, RingItem};
use std::fs::File;

fn main() {
    let f = File::open("run-0446-00.evt").unwrap();
    let m = unsafe { Mmap::map(&f) }.unwrap();

    let d = NsclData::new(&m);
    for e in d.filter(|e| matches!(e.item_type(), 30)) {
        let bh = e.body_header();
        println!("Event Size: {}", e.size());
        println!("Event Item Type: {}", e.item_type());
        println!("BodyHeader Size: {}", bh.size());
        println!("BodyHeader Timestamp: {:?}", bh.timestamp());
        println!("BodyHeader SourceID: {:?}", bh.source_id());
        println!("BodyHeader Barrier Type: {:?}", bh.barrier_type());
        match e.ring_item() {
            RingItem::BeginRun(ri) => {
                println!("BeginRun Run Number: {}", ri.run_number());
                println!("BeginRun Time Offset: {}", ri.time_offset());
                println!("BeginRun Timestamp: {}", ri.timestamp());
                println!("BeginRun Offset Divisor: {}", ri.offset_divisor());
                println!("BeginRun Title Bytes: {:02x?}", ri.title_bytes());
                println!("BeginRun Title: {}", ri.title());
            }
            RingItem::EndRun(ri) => {
                println!("EndRun Run Number: {}", ri.run_number());
                println!("EndRun Time Offset: {}", ri.time_offset());
                println!("EndRun Timestamp: {}", ri.timestamp());
                println!("EndRun Offset Divisor: {}", ri.offset_divisor());
                println!("EndRun Title Bytes: {:02x?}", ri.title_bytes());
                println!("EndRun Title: {}", ri.title());
            }
            RingItem::PauseRun(ri) => {
                println!("PauseRun Run Number: {}", ri.run_number());
                println!("PauseRun Time Offset: {}", ri.time_offset());
                println!("PauseRun Timestamp: {}", ri.timestamp());
                println!("PauseRun Offset Divisor: {}", ri.offset_divisor());
                println!("PauseRun Title Bytes: {:02x?}", ri.title_bytes());
                println!("PauseRun Title: {}", ri.title());
            }
            RingItem::ResumeRun(ri) => {
                println!("ResumeRun Run Number: {}", ri.run_number());
                println!("ResumeRun Time Offset: {}", ri.time_offset());
                println!("ResumeRun Timestamp: {}", ri.timestamp());
                println!("ResumeRun Offset Divisor: {}", ri.offset_divisor());
                println!("ResumeRun Title Bytes: {:02x?}", ri.title_bytes());
                println!("ResumeRun Title: {}", ri.title());
            }
            RingItem::AbnormalEndRun(ri) => {
                println!("AbnormalEndRun Run Number: {}", ri.run_number());
                println!("AbnormalEndRun Time Offset: {}", ri.time_offset());
                println!("AbnormalEndRun Timestamp: {}", ri.timestamp());
                println!("AbnormalEndRun Offset Divisor: {}", ri.offset_divisor());
                println!("AbnormalEndRun Title Bytes: {:02x?}", ri.title_bytes());
                println!("AbnormalEndRun Title: {}", ri.title());
            }
            RingItem::PacketTypes(ri) => {
                println!("PacketTypes Time Offset: {}", ri.time_offset());
                println!("PacketTypes Timestamp: {}", ri.timestamp());
                println!("PacketTypes String Count: {}", ri.string_count());
                println!("PacketTypes Offset Divisor: {}", ri.offset_divisor());
                println!("PacketTypes Strings Bytes: {:02x?}", ri.strings_bytes());
                println!("PacketTypes Strings: {:?}", ri.strings());
            }
            RingItem::MonitoredVariables(ri) => {
                println!("MonitoredVariables Time Offset: {}", ri.time_offset());
                println!("MonitoredVariables Timestamp: {}", ri.timestamp());
                println!("MonitoredVariables String Count: {}", ri.string_count());
                println!("MonitoredVariables Offset Divisor: {}", ri.offset_divisor());
                println!(
                    "MonitoredVariables Strings Bytes: {:02x?}",
                    ri.strings_bytes()
                );
                println!("MonitoredVariables Strings: {:?}", ri.strings());
            }
            RingItem::RingFormat(ri) => {
                println!("Data Format: ({}, {})", ri.major(), ri.minor());
            }
            RingItem::PeriodicScalers(ri) => {
                println!(
                    "PeriodicScalers Interval Start Offset: {}",
                    ri.interval_start_offset()
                );
                println!(
                    "PeriodicScalers Interval End Offset: {}",
                    ri.interval_end_offset()
                );
                println!("PeriodicScalers Timestamp: {}", ri.timestamp());
                println!(
                    "PeriodicScalers Interval Divisor: {}",
                    ri.interval_divisor()
                );
                println!("PeriodicScalers Scaler Count: {}", ri.scaler_count());
                println!("PeriodicScalers Is Incremental: {}", ri.is_incremental());
                println!("PeriodicScalers Scalers: {:?}", ri.scalers());
            }
            RingItem::PhysicsEvent(_ri) => {}
            RingItem::PhysicsEventCount(ri) => {
                println!("PhysicsEventCount Time Offset: {}", ri.time_offset());
                println!("PhysicsEventCount Offset Divisor: {}", ri.offset_divisor());
                println!("PhysicsEventCount Timestamp: {}", ri.timestamp());
                println!("PhysicsEventCount Event Count: {}", ri.event_count());
            }
            RingItem::EvbFragment(_ri) => {}
            RingItem::EvbUnknownPayload(_ri) => {}
            RingItem::EvbGlomInfo(ri) => {
                println!("EvbGlomInfo Coincident Ticks: {}", ri.coincident_ticks());
                println!("EvbGlomInfo Is Building: {}", ri.is_building());
                println!("EvbGlomInfo Timestamp Policy: {}", ri.timestamp_policy());
            }
            RingItem::UserItem(_ri) => {}
        }
        println!();
    }
}
