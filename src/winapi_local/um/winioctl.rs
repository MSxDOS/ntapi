#[cfg_attr(not(feature = "aggressive-inline"), inline)]
#[cfg_attr(feature = "aggressive-inline", inline(always))]
pub(crate) const fn CTL_CODE(DeviceType: u32, Function: u32, Method: u32, Access: u32) -> u32 {
    (DeviceType << 16) | (Access << 14) | (Function << 2) | Method
}
