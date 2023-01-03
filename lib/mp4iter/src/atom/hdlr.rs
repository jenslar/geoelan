//! Media handler atom (`hdlr`).

/// Media handler atom.
#[derive(Debug, Default)]
pub struct Hdlr {
    /// Byte 12-15
    pub component_type: u32,
    /// Four CC for the type of media or data handler
    pub component_sub_type: u32,
    /// Reserved, should be set to 0.
    pub component_manufacturer: u32,
    /// Reserved, should be set to 0.
    pub component_flags: u32,
    /// Reserved, should be set to 0.
    pub component_flags_mask: u32,
    /// Counted string. Specifies the name of the component.
    /// May contain a zero-length (empty) string.
    /// First byte contains length of string.
    /// For e.g. GoPro MP4 it's padded with `0x20` (space)
    pub component_name: String,
}