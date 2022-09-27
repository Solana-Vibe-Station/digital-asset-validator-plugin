// automatically generated by the FlatBuffers compiler, do not modify

extern crate flatbuffers;

#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_STATUS: i8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_STATUS: i8 = 2;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_STATUS: [Status; 3] = [Status::Processed, Status::Rooted, Status::Confirmed];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Status(pub i8);
#[allow(non_upper_case_globals)]
impl Status {
    pub const Processed: Self = Self(0);
    pub const Rooted: Self = Self(1);
    pub const Confirmed: Self = Self(2);

    pub const ENUM_MIN: i8 = 0;
    pub const ENUM_MAX: i8 = 2;
    pub const ENUM_VALUES: &'static [Self] = &[Self::Processed, Self::Rooted, Self::Confirmed];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Processed => Some("Processed"),
            Self::Rooted => Some("Rooted"),
            Self::Confirmed => Some("Confirmed"),
            _ => None,
        }
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl<'a> flatbuffers::Follow<'a> for Status {
    type Inner = Self;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { flatbuffers::read_scalar_at::<i8>(buf, loc) };
        Self(b)
    }
}

impl flatbuffers::Push for Status {
    type Output = Status;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe {
            flatbuffers::emplace_scalar::<i8>(dst, self.0);
        }
    }
}

impl flatbuffers::EndianScalar for Status {
    #[inline]
    fn to_little_endian(self) -> Self {
        let b = i8::to_le(self.0);
        Self(b)
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(self) -> Self {
        let b = i8::from_le(self.0);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for Status {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        i8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for Status {}
pub enum SlotStatusInfoOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SlotStatusInfo<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SlotStatusInfo<'a> {
    type Inner = SlotStatusInfo<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> SlotStatusInfo<'a> {
    pub const VT_SLOT: flatbuffers::VOffsetT = 4;
    pub const VT_PARENT: flatbuffers::VOffsetT = 6;
    pub const VT_STATUS: flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        SlotStatusInfo { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args SlotStatusInfoArgs,
    ) -> flatbuffers::WIPOffset<SlotStatusInfo<'bldr>> {
        let mut builder = SlotStatusInfoBuilder::new(_fbb);
        if let Some(x) = args.parent {
            builder.add_parent(x);
        }
        builder.add_slot(args.slot);
        builder.add_status(args.status);
        builder.finish()
    }

    #[inline]
    pub fn slot(&self) -> u64 {
        self._tab
            .get::<u64>(SlotStatusInfo::VT_SLOT, Some(0))
            .unwrap()
    }
    #[inline]
    pub fn parent(&self) -> Option<u64> {
        self._tab.get::<u64>(SlotStatusInfo::VT_PARENT, None)
    }
    #[inline]
    pub fn status(&self) -> Status {
        self._tab
            .get::<Status>(SlotStatusInfo::VT_STATUS, Some(Status::Processed))
            .unwrap()
    }
}

impl flatbuffers::Verifiable for SlotStatusInfo<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        v.visit_table(pos)?
            .visit_field::<u64>("slot", Self::VT_SLOT, false)?
            .visit_field::<u64>("parent", Self::VT_PARENT, false)?
            .visit_field::<Status>("status", Self::VT_STATUS, false)?
            .finish();
        Ok(())
    }
}
pub struct SlotStatusInfoArgs {
    pub slot: u64,
    pub parent: Option<u64>,
    pub status: Status,
}
impl<'a> Default for SlotStatusInfoArgs {
    #[inline]
    fn default() -> Self {
        SlotStatusInfoArgs {
            slot: 0,
            parent: None,
            status: Status::Processed,
        }
    }
}

pub struct SlotStatusInfoBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SlotStatusInfoBuilder<'a, 'b> {
    #[inline]
    pub fn add_slot(&mut self, slot: u64) {
        self.fbb_.push_slot::<u64>(SlotStatusInfo::VT_SLOT, slot, 0);
    }
    #[inline]
    pub fn add_parent(&mut self, parent: u64) {
        self.fbb_
            .push_slot_always::<u64>(SlotStatusInfo::VT_PARENT, parent);
    }
    #[inline]
    pub fn add_status(&mut self, status: Status) {
        self.fbb_
            .push_slot::<Status>(SlotStatusInfo::VT_STATUS, status, Status::Processed);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SlotStatusInfoBuilder<'a, 'b> {
        let start = _fbb.start_table();
        SlotStatusInfoBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<SlotStatusInfo<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for SlotStatusInfo<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("SlotStatusInfo");
        ds.field("slot", &self.slot());
        ds.field("parent", &self.parent());
        ds.field("status", &self.status());
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_slot_status_info<'a>(buf: &'a [u8]) -> SlotStatusInfo<'a> {
    unsafe { flatbuffers::root_unchecked::<SlotStatusInfo<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_slot_status_info<'a>(buf: &'a [u8]) -> SlotStatusInfo<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<SlotStatusInfo<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `SlotStatusInfo`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_slot_status_info_unchecked`.
pub fn root_as_slot_status_info(
    buf: &[u8],
) -> Result<SlotStatusInfo, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<SlotStatusInfo>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `SlotStatusInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_slot_status_info_unchecked`.
pub fn size_prefixed_root_as_slot_status_info(
    buf: &[u8],
) -> Result<SlotStatusInfo, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<SlotStatusInfo>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `SlotStatusInfo` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_slot_status_info_unchecked`.
pub fn root_as_slot_status_info_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<SlotStatusInfo<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<SlotStatusInfo<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `SlotStatusInfo` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_slot_status_info_unchecked`.
pub fn size_prefixed_root_as_slot_status_info_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<SlotStatusInfo<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<SlotStatusInfo<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a SlotStatusInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `SlotStatusInfo`.
pub unsafe fn root_as_slot_status_info_unchecked(buf: &[u8]) -> SlotStatusInfo {
    flatbuffers::root_unchecked::<SlotStatusInfo>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed SlotStatusInfo and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `SlotStatusInfo`.
pub unsafe fn size_prefixed_root_as_slot_status_info_unchecked(buf: &[u8]) -> SlotStatusInfo {
    flatbuffers::size_prefixed_root_unchecked::<SlotStatusInfo>(buf)
}
#[inline]
pub fn finish_slot_status_info_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<SlotStatusInfo<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_slot_status_info_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<SlotStatusInfo<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
