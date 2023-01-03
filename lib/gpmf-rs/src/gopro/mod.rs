//! Various GoPro related structs and methods.

pub mod device_name;
pub mod device_id;
pub mod file;
pub mod session;
pub mod media_id;
pub mod meta;
pub mod recording_type;

pub use file::GoProFile;
pub use session::GoProSession;
pub use meta::GoProMeta;
pub use device_id::Dvid;
pub use media_id::Muid;
pub use device_name::DeviceName;
pub use recording_type::RecordingType;