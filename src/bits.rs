use std::array::TryFromSliceError;

pub(crate) trait TryFromSlice<const N: usize>
where
    Self: Sized,
{
    fn try_from_slice(slice: &[u8], start: usize) -> Result<Self, TryFromSliceError>;
}

impl TryFromSlice<1> for u8 {
    fn try_from_slice(slice: &[u8], start: usize) -> Result<Self, TryFromSliceError> {
        slice[start..][..1]
            .try_into()
            .map(|x| Self::from_le_bytes(x))
    }
}
impl TryFromSlice<2> for u16 {
    fn try_from_slice(slice: &[u8], start: usize) -> Result<Self, TryFromSliceError> {
        slice[start..][..2]
            .try_into()
            .map(|x| Self::from_le_bytes(x))
    }
}

impl TryFromSlice<4> for u32 {
    fn try_from_slice(slice: &[u8], start: usize) -> Result<Self, TryFromSliceError> {
        slice[start..][..4]
            .try_into()
            .map(|x| Self::from_le_bytes(x))
    }
}

impl TryFromSlice<8> for u64 {
    fn try_from_slice(slice: &[u8], start: usize) -> Result<Self, TryFromSliceError> {
        slice[start..][..8]
            .try_into()
            .map(|x| Self::from_le_bytes(x))
    }
}
