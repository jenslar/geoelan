// #![allow(dead_code)]

// extern crate byteorder;
// use byteorder::{ByteOrder, LittleEndian};

// pub fn get_baselength(basetype: u8) -> u8 {
//     match basetype {
//         // basetype id, individual value length (e.g. 134 is u32 -> 4 bytes)
//         0 => 1, // ENUM, u8
//         1 => 1, // SINT8, u8
//         2 => 1, // UINT8, u8
//         7 => 128,// LENGTH NOT USED (NO LENGTH IN PROFILE.XLSX, STRING SO LENGTH VARIES)
//         10 => 1, // UINT8Z, u8
//         13 => 1, // BYTE, u8
//         131 => 2, // SINT16, i16
//         132 => 2, // UINT16, u16
//         133 => 4, // SINT32, i32
//         134 => 4, // UINT32, u32
//         136 => 4, // FLOAT32, f32
//         137 => 8, // FLOAT64, f64
//         139 => 2, // UINT16Z, u16
//         140 => 4, // UINT32Z, u32
//         142 => 8, // SINT64, i64
//         143 => 8, // UINT64, u64
//         144 => 8, // UIN64Z, u64
//         // _ => panic!("(!) [LENGTH] Basetype {} not found", basetype)
//         _ => 0 // for error handling
//     }
// }

// #[derive(Debug)]
// pub enum BaseType {
//     ENUM(u8),
//     SINT8(i8),
//     UINT8(u8),
//     STRING(String),
//     BYTE(u8),
//     UINT8Z(u8),
//     SINT16(i16),
//     UINT16(u16),
//     SINT32(i32),
//     UINT32(u32),
//     FLOAT32(f32),
//     FLOAT64(f64),
//     UINT16Z(u16), // Z = ?
//     UINT32Z(u32), // Z = ?
//     SINT64(i64),
//     UINT64(u64),
//     UINT64Z(u64), // Z = ?
//     NONE, // for error handling
// }

// pub fn get_basevalue(basetype: u8, data: &[u8]) -> BaseType {
//     // See Profile.xlsx: Types, fit_base_types
//     match basetype {
//         0 => BaseType::ENUM(data[0]), // byte
//         1 => BaseType::SINT8(data[0] as i8),
//         2 => BaseType::UINT8(data[0]),
//         7 => {
//             let mut data_str = String::new();
//             for &c in data {if c != 0 {data_str.push(c as char)}}; // disregard ascii null
//             BaseType::STRING(data_str)
//         },
//         10 => BaseType::UINT8Z(data[0]), // Z = ?
//         13 => BaseType::BYTE(data[0]), // byte
//         131 => BaseType::SINT16(LittleEndian::read_i16(data)),
//         132 => BaseType::UINT16(LittleEndian::read_u16(data)),
//         133 => BaseType::SINT32(LittleEndian::read_i32(data)),
//         134 => BaseType::UINT32(LittleEndian::read_u32(data)),
//         136 => BaseType::FLOAT32(LittleEndian::read_f32(data)),
//         137 => BaseType::FLOAT64(LittleEndian::read_f64(data)),
//         139 => BaseType::UINT16Z(LittleEndian::read_u16(data)), // Z = ?
//         140 => BaseType::UINT32Z(LittleEndian::read_u32(data)), // Z = ?
//         142 => BaseType::SINT64(LittleEndian::read_i64(data)),
//         143 => BaseType::UINT64(LittleEndian::read_u64(data)),
//         144 => BaseType::UINT64Z(LittleEndian::read_u64(data)), // Z = ?
//         // _ => panic!("(!) [VALUE] Basetype {} not found", basetype)
//         _ => BaseType::NONE, // for error handling
//     }
// }
