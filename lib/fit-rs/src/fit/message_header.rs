//! FIT message header.

use binread::BinRead;

use crate::Fit;

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Definition,
    Data
}

/// FIT message header.
#[derive(Debug, Copy, Clone, BinRead)]
pub struct MessageHeader(u8);

impl MessageHeader {
    /// Checks whether message is a definition
    /// or a data message.
    pub fn kind(&self) -> MessageType {
        match Fit::bit_set(self.0, 6) {
            true => MessageType::Definition,
            false => MessageType::Data,
        }
    }

    /// FIT local ID (0-15) for matching
    /// definition messages with data message
    /// during parse.
    /// 
    /// Note that while `id()` supports compressed
    /// timestamp headers the rest of `fit-rs` does not.
    pub fn id(&self) -> u8 {
        // check for compressed time stamp header
        match self.comp_time() {
            Some(_) => (0b0110_0000 & self.0) >> 5,
            None => 0b0000_1111 & self.0
        }
    }

    /// Checks if header is a compressed
    /// time stamp header and returns the
    /// value if so.
    /// 
    /// Note that while `comp_time()` supports compressed
    /// timestamp headers the rest of `fit-rs` does not.
    pub fn comp_time(&self) -> Option<u8> {
        match Fit::bit_set(self.0, 7) {
            true => Some(0b0001_1111 & self.0),
            false => None
        }
    }

    /// Returns `true` if definition contains developer fields.
    /// Only relvant for definition messages and always returns `false`
    /// for data messages.
    pub fn dev_fields(&self) -> bool {
        match self.kind() {
            MessageType::Definition => Fit::bit_set(self.0, 5),
            MessageType::Data => false
        }
    }
}