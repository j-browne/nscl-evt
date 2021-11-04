#![feature(bench_black_box)]
use nscl_evt::{NsclData, RingItem};
use std::{fs::File, hint::black_box, io::Read};

fn main() {
    let mut f = File::open("run-0446-00.evt").unwrap();
    let mut v = Vec::with_capacity(f.metadata().unwrap().len() as usize);
    let _ = f.read_to_end(&mut v).unwrap();

    let d = NsclData::new(&v);
    d.into_iter().for_each(|e| {
        let _ = black_box(e.bytes());
        let _ = black_box(e.size());
        let _ = black_box(e.type_id());
        let bh = black_box(e.body_header());
        let _ = black_box(bh.bytes());
        let _ = black_box(bh.size());
        let _ = black_box(bh.timestamp());
        let _ = black_box(bh.source_id());
        let _ = black_box(bh.barrier_type());
        match e.ring_item() {
            RingItem::BeginRun(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.run_number());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.title_bytes());
                let _ = black_box(ri.title());
            }
            RingItem::EndRun(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.run_number());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.title_bytes());
                let _ = black_box(ri.title());
            }
            RingItem::PauseRun(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.run_number());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.title_bytes());
                let _ = black_box(ri.title());
            }
            RingItem::ResumeRun(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.run_number());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.title_bytes());
                let _ = black_box(ri.title());
            }
            RingItem::AbnormalEndRun(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.run_number());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.title_bytes());
                let _ = black_box(ri.title());
            }
            RingItem::PacketTypes(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.string_count());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.strings_bytes());
                let _ = black_box(ri.strings());
            }
            RingItem::MonitoredVariables(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.string_count());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.strings_bytes());
                let _ = black_box(ri.strings());
            }
            RingItem::RingFormat(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.major());
                let _ = black_box(ri.minor());
            }
            RingItem::PeriodicScalers(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.interval_start_offset());
                let _ = black_box(ri.interval_end_offset());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.interval_divisor());
                let _ = black_box(ri.scaler_count());
                let _ = black_box(ri.is_incremental());
                let _ = black_box(ri.scalers());
            }
            RingItem::PhysicsEvent(ri) => {
                let _ = black_box(ri.bytes());
            }
            RingItem::PhysicsEventCount(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.time_offset());
                let _ = black_box(ri.offset_divisor());
                let _ = black_box(ri.timestamp());
                let _ = black_box(ri.event_count());
            }
            RingItem::EvbFragment(ri) => {
                let _ = black_box(ri.bytes());
            }
            RingItem::EvbUnknownPayload(ri) => {
                let _ = black_box(ri.bytes());
            }
            RingItem::EvbGlomInfo(ri) => {
                let _ = black_box(ri.bytes());
                let _ = black_box(ri.coincident_ticks());
                let _ = black_box(ri.is_building());
                let _ = black_box(ri.timestamp_policy());
            }
            RingItem::UserItem(ri) => {
                let _ = black_box(ri.bytes());
            }
        }
    });
}
