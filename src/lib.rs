#![allow(dead_code)]
// TODO: Handle unwraps?
// TODO: Limit length of source slices?

use bits::TryFromSlice;
mod bits;

#[derive(Debug, Clone, Copy)]
pub struct NsclData<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> NsclData<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source, cursor: 0 }
    }
}

impl<'s> Iterator for NsclData<'s> {
    type Item = Event<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.source.len() {
            None
        } else {
            let event = Event {
                source: self.source,
                cursor: self.cursor,
            };
            // If an event has a size of 0, we won't be able to progress
            assert_ne!(event.size(), 0);
            self.cursor += event.size() as usize;
            Some(event)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Event<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> Event<'s> {
    pub fn size(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn item_type(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 4).unwrap()
    }

    pub fn body_header(&self) -> BodyHeader {
        BodyHeader {
            source: self.source,
            cursor: self.cursor + 8,
        }
    }

    pub fn ring_item(&self) -> RingItem<'s> {
        let offset = match self.body_header().size() {
            0 => 12 as usize,
            x => x as usize + 8,
        };

        match self.item_type() {
            1 => RingItem::BeginRun(StateChange {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            2 => RingItem::EndRun(StateChange {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            3 => RingItem::PauseRun(StateChange {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            4 => RingItem::ResumeRun(StateChange {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            5 => RingItem::AbnormalEndRun(StateChange {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            10 => RingItem::PacketTypes(Text {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            11 => RingItem::MonitoredVariables(Text {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            12 => RingItem::RingFormat(RingFormat {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            20 => RingItem::PeriodicScalers(PeriodicScalers {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            30 => RingItem::PhysicsEvent(PhysicsEvent {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            31 => RingItem::PhysicsEventCount(PhysicsEventCount {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            40 => RingItem::EvbFragment(EvbFragment {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            41 => RingItem::EvbUnknownPayload(EvbUnknownPayload {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            42 => RingItem::EvbGlomInfo(EvbGlomInfo {
                source: self.source,
                cursor: self.cursor + offset,
            }),
            x if x > 32768 => unimplemented!("user items not implemented"),
            _ => panic!("unknown ring item type"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BodyHeader<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> BodyHeader<'s> {
    pub fn size(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn timestamp(&self) -> Option<u64> {
        match self.size() {
            0 => None,
            20 => Some(u64::try_from_slice(self.source, self.cursor + 4).unwrap()),
            _ => panic!("bad BodyHeader size"),
        }
    }

    pub fn source_id(&self) -> Option<u32> {
        match self.size() {
            0 => None,
            20 => Some(u32::try_from_slice(self.source, self.cursor + 12).unwrap()),
            _ => panic!("bad BodyHeader size"),
        }
    }

    pub fn barrier_type(&self) -> Option<u32> {
        match self.size() {
            0 => None,
            20 => Some(u32::try_from_slice(self.source, self.cursor + 16).unwrap()),
            _ => panic!("bad BodyHeader size"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RingItem<'s> {
    BeginRun(StateChange<'s>),
    EndRun(StateChange<'s>),
    PauseRun(StateChange<'s>),
    ResumeRun(StateChange<'s>),
    AbnormalEndRun(StateChange<'s>),
    PacketTypes(Text<'s>),
    MonitoredVariables(Text<'s>),
    RingFormat(RingFormat<'s>),
    PeriodicScalers(PeriodicScalers<'s>),
    PhysicsEvent(PhysicsEvent<'s>),
    PhysicsEventCount(PhysicsEventCount<'s>),
    EvbFragment(EvbFragment<'s>),
    EvbUnknownPayload(EvbUnknownPayload<'s>),
    EvbGlomInfo(EvbGlomInfo<'s>),
    UserItem(UserItem<'s>),
}

impl<'s> RingItem<'s> {
    pub fn as_begin_run(&self) -> Option<StateChange<'s>> {
        match *self {
            Self::BeginRun(ri) => Some(ri),
            _ => None,
        }
    }

    pub fn as_end_run(&self) -> Option<StateChange<'s>> {
        match *self {
            Self::EndRun(ri) => Some(ri),
            _ => None,
        }
    }

    pub fn as_pause_run(&self) -> Option<StateChange<'s>> {
        match *self {
            Self::PauseRun(ri) => Some(ri),
            _ => None,
        }
    }

    pub fn as_resume_run(&self) -> Option<StateChange<'s>> {
        match *self {
            Self::ResumeRun(ri) => Some(ri),
            _ => None,
        }
    }

    pub fn as_abnormal_end_run(&self) -> Option<StateChange<'s>> {
        match *self {
            Self::AbnormalEndRun(ri) => Some(ri),
            _ => None,
        }
    }

    pub fn as_packet_types(&self) -> Option<Text<'s>> {
        match *self {
            Self::PacketTypes(ri) => Some(ri),
            _ => None,
        }
    }

    pub fn as_monitored_variables(&self) -> Option<Text<'s>> {
        match *self {
            Self::MonitoredVariables(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_ring_format(&self) -> Option<RingFormat<'s>> {
        match *self {
            Self::RingFormat(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_periodic_scalers(&self) -> Option<PeriodicScalers<'s>> {
        match *self {
            Self::PeriodicScalers(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_physics_event(&self) -> Option<PhysicsEvent<'s>> {
        match *self {
            Self::PhysicsEvent(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_physics_event_count(&self) -> Option<PhysicsEventCount<'s>> {
        match *self {
            Self::PhysicsEventCount(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_evb_fragment(&self) -> Option<EvbFragment<'s>> {
        match *self {
            Self::EvbFragment(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_evb_unknown_payload(&self) -> Option<EvbUnknownPayload<'s>> {
        match *self {
            Self::EvbUnknownPayload(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_evb_glom_info(&self) -> Option<EvbGlomInfo<'s>> {
        match *self {
            Self::EvbGlomInfo(ri) => Some(ri),
            _ => None,
        }
    }
    pub fn as_user_item(&self) -> Option<UserItem<'s>> {
        match *self {
            Self::UserItem(ri) => Some(ri),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StateChange<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> StateChange<'s> {
    pub fn run_number(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn time_offset(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 4).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 8).unwrap()
    }

    pub fn offset_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 12).unwrap()
    }

    pub fn title_bytes(&self) -> &[u8] {
        &self.source[self.cursor + 16..][..80]
    }

    pub fn title(&self) -> &str {
        let end = self.source[self.cursor + 16..]
            .iter()
            .position(|x| *x == 0)
            .unwrap();
        std::str::from_utf8(&self.source[self.cursor + 16..][..end]).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Text<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> Text<'s> {
    pub fn time_offset(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 4).unwrap()
    }

    pub fn string_count(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 8).unwrap()
    }

    pub fn offset_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 12).unwrap()
    }

    pub fn strings_bytes(&self) -> &[u8] {
        let n = self.string_count();
        let mut end = 0;
        for _ in 0..n {
            end += self.source[self.cursor + 16 + end..]
                .iter()
                .position(|x| *x == 0)
                .unwrap()
                + 1;
        }
        &self.source[self.cursor + 16..][..end]
    }

    pub fn strings(&self) -> Vec<&str> {
        let n = self.string_count();
        self.source[self.cursor + 16..]
            .split(|x| *x == 0)
            .take(n as usize)
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RingFormat<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> RingFormat<'s> {
    pub fn major(&self) -> u16 {
        u16::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn minor(&self) -> u16 {
        u16::try_from_slice(self.source, self.cursor + 2).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PeriodicScalers<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> PeriodicScalers<'s> {
    pub fn interval_start_offset(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn interval_end_offset(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 4).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 8).unwrap()
    }

    pub fn interval_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 12).unwrap()
    }

    pub fn scaler_count(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 16).unwrap()
    }

    pub fn is_incremental(&self) -> bool {
        match u32::try_from_slice(self.source, self.cursor + 20).unwrap() {
            0 => false,
            _ => true,
        }
    }

    pub fn scalers(&self) -> Vec<u32> {
        (0..self.scaler_count() as usize)
            .map(|i| u32::try_from_slice(self.source, self.cursor + 24 + i * 4).unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicsEvent<'s> {
    source: &'s [u8],
    cursor: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicsEventCount<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> PhysicsEventCount<'s> {
    pub fn time_offset(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn offset_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 4).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, self.cursor + 8).unwrap()
    }

    pub fn event_count(&self) -> u64 {
        u64::try_from_slice(self.source, self.cursor + 12).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EvbFragment<'s> {
    source: &'s [u8],
    cursor: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct EvbUnknownPayload<'s> {
    source: &'s [u8],
    cursor: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct EvbGlomInfo<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> EvbGlomInfo<'s> {
    pub fn coincident_ticks(&self) -> u64 {
        u64::try_from_slice(self.source, self.cursor + 0).unwrap()
    }

    pub fn is_building(&self) -> bool {
        match u16::try_from_slice(self.source, self.cursor + 8).unwrap() {
            0 => false,
            _ => true,
        }
    }

    pub fn timestamp_policy(&self) -> u16 {
        u16::try_from_slice(self.source, self.cursor + 10).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UserItem<'s> {
    source: &'s [u8],
    cursor: usize,
}

impl<'s> UserItem<'s> {
    pub fn data(&self) -> &[u8] {
        unimplemented!()
    }
}
