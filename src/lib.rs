#![allow(dead_code)]
// TODO: Handle unwraps?
// TODO: Handle out of bounds?

use bits::TryFromSlice;
mod bits;

#[derive(Debug, Clone, Copy)]
pub struct NsclData<'s> {
    source: &'s [u8],
}

impl<'s> NsclData<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }
}

impl<'s> Iterator for NsclData<'s> {
    type Item = Event<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_empty() {
            None
        } else {
            let event = Event::new(self.source);

            // Advance pointer
            // If event size is 0, something's wrong
            let size = event.size() as usize;
            if size == 0 {
                panic!("event size is 0")
            } else {
                self.source = &self.source[size..];
            }

            Some(event)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Event<'s> {
    source: &'s [u8],
}

impl<'s> Event<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        let size = u32::try_from_slice(source, 0).unwrap() as usize;
        Self {
            source: &source[..size],
        }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }

    pub fn size(&self) -> u32 {
        u32::try_from_slice(self.source, 0).unwrap()
    }

    pub fn type_id(&self) -> u32 {
        u32::try_from_slice(self.source, 4).unwrap()
    }

    pub fn body_header(&self) -> BodyHeader {
        BodyHeader::new(&self.source[8..])
    }

    pub fn ring_item(&self) -> RingItem<'s> {
        let offset = match self.body_header().size() {
            0 => 12 as usize,
            x => x as usize + 8,
        };
        RingItem::new(&self.source[offset..], self.type_id())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BodyHeader<'s> {
    BodyHeader0 { source: &'s [u8] },
    BodyHeader20 { source: &'s [u8] },
}

impl<'s> BodyHeader<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        let size = u32::try_from_slice(source, 0).unwrap();
        match size {
            0 => Self::BodyHeader0 {
                source: &source[..4],
            },
            20 => Self::BodyHeader20 {
                source: &source[..20],
            },
            _ => panic!("bad BodyHeader size"),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            Self::BodyHeader0 { source } | Self::BodyHeader20 { source } => source,
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Self::BodyHeader0 { source } | Self::BodyHeader20 { source } => {
                u32::try_from_slice(source, 0).unwrap()
            }
        }
    }

    pub fn timestamp(&self) -> Option<u64> {
        match self {
            Self::BodyHeader0 { .. } => None,
            Self::BodyHeader20 { source } => Some(u64::try_from_slice(source, 4).unwrap()),
        }
    }

    pub fn source_id(&self) -> Option<u32> {
        match self {
            Self::BodyHeader0 { .. } => None,
            Self::BodyHeader20 { source } => Some(u32::try_from_slice(source, 12).unwrap()),
        }
    }

    pub fn barrier_type(&self) -> Option<u32> {
        match self {
            Self::BodyHeader0 { .. } => None,
            Self::BodyHeader20 { source } => Some(u32::try_from_slice(source, 16).unwrap()),
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
    pub fn new(source: &'s [u8], type_id: u32) -> Self {
        match type_id {
            1 => Self::BeginRun(StateChange::new(source)),
            2 => Self::EndRun(StateChange::new(source)),
            3 => Self::PauseRun(StateChange::new(source)),
            4 => Self::ResumeRun(StateChange::new(source)),
            5 => Self::AbnormalEndRun(StateChange::new(source)),
            10 => Self::PacketTypes(Text::new(source)),
            11 => Self::MonitoredVariables(Text::new(source)),
            12 => Self::RingFormat(RingFormat::new(source)),
            20 => Self::PeriodicScalers(PeriodicScalers::new(source)),
            30 => Self::PhysicsEvent(PhysicsEvent::new(source)),
            31 => Self::PhysicsEventCount(PhysicsEventCount::new(source)),
            40 => Self::EvbFragment(EvbFragment::new(source)),
            41 => Self::EvbUnknownPayload(EvbUnknownPayload::new(source)),
            42 => Self::EvbGlomInfo(EvbGlomInfo::new(source)),
            x if x > 32768 => Self::UserItem(UserItem::new(source)),
            _ => panic!("unknown ring item type"),
        }
    }

    pub fn bytes(&self) -> &'s [u8] {
        match self {
            Self::BeginRun(StateChange { source })
            | Self::EndRun(StateChange { source })
            | Self::PauseRun(StateChange { source })
            | Self::ResumeRun(StateChange { source })
            | Self::AbnormalEndRun(StateChange { source })
            | Self::PacketTypes(Text { source })
            | Self::MonitoredVariables(Text { source })
            | Self::RingFormat(RingFormat { source })
            | Self::PeriodicScalers(PeriodicScalers { source })
            | Self::PhysicsEvent(PhysicsEvent { source })
            | Self::PhysicsEventCount(PhysicsEventCount { source })
            | Self::EvbFragment(EvbFragment { source })
            | Self::EvbUnknownPayload(EvbUnknownPayload { source })
            | Self::EvbGlomInfo(EvbGlomInfo { source })
            | Self::UserItem(UserItem { source }) => source,
        }
    }

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
}

impl<'s> StateChange<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }

    pub fn run_number(&self) -> u32 {
        u32::try_from_slice(self.source, 0).unwrap()
    }

    pub fn time_offset(&self) -> u32 {
        u32::try_from_slice(self.source, 4).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, 8).unwrap()
    }

    pub fn offset_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, 12).unwrap()
    }

    pub fn title_bytes(&self) -> &[u8] {
        // You could just go to the end of source, since the event should end after the string
        &self.source[16..][..80]
    }

    pub fn title(&self) -> &str {
        // Ignore bytes starting with the first NUL
        let end = self.source[16..].iter().position(|x| *x == 0).unwrap();
        std::str::from_utf8(&self.source[16..][..end]).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Text<'s> {
    source: &'s [u8],
}

impl<'s> Text<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }

    pub fn time_offset(&self) -> u32 {
        u32::try_from_slice(self.source, 0).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, 4).unwrap()
    }

    pub fn string_count(&self) -> u32 {
        u32::try_from_slice(self.source, 8).unwrap()
    }

    pub fn offset_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, 12).unwrap()
    }

    pub fn strings_bytes(&self) -> &[u8] {
        &self.source[16..]
    }

    pub fn strings(&self) -> Vec<&str> {
        let n = self.string_count();
        self.source[16..]
            .split(|x| *x == 0)
            .take(n as usize) // split leaves an empty string at the end
            .map(|x| std::str::from_utf8(x).unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RingFormat<'s> {
    source: &'s [u8],
}

impl<'s> RingFormat<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }

    pub fn major(&self) -> u16 {
        u16::try_from_slice(self.source, 0).unwrap()
    }

    pub fn minor(&self) -> u16 {
        u16::try_from_slice(self.source, 2).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PeriodicScalers<'s> {
    source: &'s [u8],
}

impl<'s> PeriodicScalers<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }

    pub fn interval_start_offset(&self) -> u32 {
        u32::try_from_slice(self.source, 0).unwrap()
    }

    pub fn interval_end_offset(&self) -> u32 {
        u32::try_from_slice(self.source, 4).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, 8).unwrap()
    }

    pub fn interval_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, 12).unwrap()
    }

    pub fn scaler_count(&self) -> u32 {
        u32::try_from_slice(self.source, 16).unwrap()
    }

    pub fn is_incremental(&self) -> bool {
        match u32::try_from_slice(self.source, 20).unwrap() {
            0 => false,
            _ => true,
        }
    }

    pub fn scalers(&self) -> Vec<u32> {
        (0..self.scaler_count() as usize)
            // Each scaler is 4 bytes long
            .map(|i| u32::try_from_slice(self.source, 24 + i * 4).unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicsEvent<'s> {
    source: &'s [u8],
}

impl<'s> PhysicsEvent<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicsEventCount<'s> {
    source: &'s [u8],
}

impl<'s> PhysicsEventCount<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }

    pub fn time_offset(&self) -> u32 {
        u32::try_from_slice(self.source, 0).unwrap()
    }

    pub fn offset_divisor(&self) -> u32 {
        u32::try_from_slice(self.source, 4).unwrap()
    }

    pub fn timestamp(&self) -> u32 {
        u32::try_from_slice(self.source, 8).unwrap()
    }

    pub fn event_count(&self) -> u64 {
        u64::try_from_slice(self.source, 12).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EvbFragment<'s> {
    source: &'s [u8],
}

impl<'s> EvbFragment<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EvbUnknownPayload<'s> {
    source: &'s [u8],
}

impl<'s> EvbUnknownPayload<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EvbGlomInfo<'s> {
    source: &'s [u8],
}

impl<'s> EvbGlomInfo<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }

    pub fn coincident_ticks(&self) -> u64 {
        u64::try_from_slice(self.source, 0).unwrap()
    }

    pub fn is_building(&self) -> bool {
        match u16::try_from_slice(self.source, 8).unwrap() {
            0 => false,
            _ => true,
        }
    }

    pub fn timestamp_policy(&self) -> u16 {
        u16::try_from_slice(self.source, 10).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UserItem<'s> {
    source: &'s [u8],
}

impl<'s> UserItem<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self { source }
    }

    pub fn bytes(&self) -> &[u8] {
        self.source
    }
}
