//! Crate for parsing [FIT-files](https://developer.garmin.com/fit/overview/), a common, binary logging  used in fitness devices from various brands. Developer data is supported. Various filtering methods and custom errors are implemented, including partial data reads. Additionally, this crate was developed in parallel with the VIRB centric tool [GeoELAN](https://gitlab.com/rwaai/geoelan) and contains some VIRB specific functions and methods that may not apply to other devices.
#![allow(dead_code)]
#![warn(rust_2018_idioms, missing_copy_implementations)]
// #![warn(rust_2018_idioms, missing_docs, missing_copy_implementations)]

/// Parse and process FIT-files in various ways.
pub mod errors;
pub mod messages; // message lists (names used in Profile.xlsx)
pub mod process; // various fit data processing algorithms (mag, acc, gyro, bar)
pub mod profile; // message type lists (names used in Profile.xlsx)
pub mod structs; // various structs for messages (names used in Profile.xlsx) // Fit parse errors

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use chrono::Duration;
use errors::Mp4Error;
use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use structs::ParseMethod; // for gps_metadata -> point conversion

/// Read 'bytes_to_read' at 'offset' in 'file'
fn read_bytes(file: &mut File, offset: u64, bytes_to_read: u64) -> std::io::Result<Vec<u8>> {
    // Unsure whether number of read bytes etc should be returned or discarded.
    // let new_offset = file.seek(SeekFrom::Start(offset))?;
    file.seek(SeekFrom::Start(offset))?;
    let mut chunk = file.take(bytes_to_read);
    // let bytes_read = chunk.read_to_end(&mut data)?; // return Result<> instead?
    let mut data = Vec::new();
    chunk.read_to_end(&mut data)?; // return Result<> instead?
                                   // Ok((new_offset, bytes_read, data))
    Ok(data)
}

/// For checking bits set in message headers.
/// - Developer fields: pos idx 5 == 1
/// - Definition message: pos idx 6 == 1
/// - Data message: pos idx 6 == 0
/// - Compressed timestamp: pos idx 7 == 1
fn bit_set(byte: u8, position: u8) -> bool {
    byte & (1 << position) != 0
}

/// Takes a u8 (FIT BaseType) and returns one of the structs::BaseType
/// 'data' is expected to be correct length/multiple for the corresponding data type (e.g. 2 for u16)
/// FIT Basetype overview
/// | Bit | Name             | Description |
/// |---|--- |---|
/// | 7   | Endian Ability     | 0 - for single byte data |
/// | |                     | 1 - if base type has endianness (i.e. base type is 2 or more bytes) |
/// | 5-6 | Reserved           | Reserved |
/// | 0-4 | Base Type Number   | Number assigned to Base Type (provided in SDK) |
fn get_basevalues(
    basetype: u8,
    data: &[u8], // all values in a single field
    architecture: u8,
    unchecked_string: bool, // only for unchecked utf-8 strings
) -> Result<structs::BaseType, errors::ParseError> {
    // See Profile.xlsx: Types, fit_base_types
    let base_type_number = 0b0001_1111 & basetype; // as in fit sdk, but ok to just use the full u8?
    let base_type_len = match &base_type_number {
        // match twice, performance issue?
        3 | 4 | 11 => 2,       // 16-bit values
        5 | 6 | 8 | 12 => 4,   // 32-bit values
        9 | 14 | 15 | 16 => 8, // 64-bit values
        _ => 1,
    };
    if data.len() % base_type_len != 0 {
        // need to handle error here since byteorder
        // read_X_into() panics on incorrect buf length
        return Err(errors::ParseError::InvalidLengthForBasetypeCluster((
            data.len(),
            base_type_number,
            base_type_len,
        )));
    }
    match base_type_number {
        0 => Ok(structs::BaseType::ENUM(data.into())), // byte -> u8
        1 => Ok(structs::BaseType::SINT8(
            data.to_vec().iter().map(|d| *d as i8).collect::<Vec<i8>>(), // cast ok?
        )),
        2 => Ok(structs::BaseType::UINT8(data.into())),
        3 => Ok(structs::BaseType::SINT16({
            let mut buf: Vec<i16> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_i16_into(data, &mut buf),
                1 => BigEndian::read_i16_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        4 => Ok(structs::BaseType::UINT16({
            let mut buf: Vec<u16> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_u16_into(data, &mut buf),
                1 => BigEndian::read_u16_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        5 => Ok(structs::BaseType::SINT32({
            let mut buf: Vec<i32> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_i32_into(data, &mut buf),
                1 => BigEndian::read_i32_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        6 => Ok(structs::BaseType::UINT32({
            let mut buf: Vec<u32> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_u32_into(data, &mut buf),
                1 => BigEndian::read_u32_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        8 => Ok(structs::BaseType::FLOAT32({
            let mut buf: Vec<f32> = vec![0.0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_f32_into(data, &mut buf),
                1 => BigEndian::read_f32_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        9 => Ok(structs::BaseType::FLOAT64({
            let mut buf: Vec<f64> = vec![0.0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_f64_into(data, &mut buf),
                1 => BigEndian::read_f64_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        7 => Ok(structs::BaseType::STRING(
            // Null terminated utf-8
            // Old behaviour: use ENTIRE LENGTH of &[u8] THEN trim null,
            // which caused parse error for some fit (FitCSVtool seems to parse this way)
            // Current behaviour: only use UP UNTIL first encountered null OR
            // if none encountered use entire length of &[u8]
            {
                if unchecked_string {
                    unsafe {
                        // Using the entire &[u8] fails for some fit...
                        // std::str::from_utf8_unchecked(data)
                        //     .trim_matches(char::from(0)) // unnecessary?
                        //     .to_string()
                        // ...this one doesn't. Which is preferred for debug?
                        if let Some(idx) = data.iter().position(|&x| x == 0) {
                            std::str::from_utf8_unchecked(&data[..idx])
                                // .trim_matches(char::from(0)) // unnecessary?
                                .to_string()
                        } else {
                            std::str::from_utf8_unchecked(data)
                                // .trim_matches(char::from(0)) // unnecessary since checked for 0?
                                .to_string()
                        }
                    }
                } else {
                    if let Some(idx) = data.iter().position(|&x| x == 0) {
                        std::str::from_utf8(&data[..idx])?
                            // .trim_matches(char::from(0)) // unnecessary?
                            .to_string()
                    } else {
                        std::str::from_utf8(data)?
                            // .trim_matches(char::from(0)) // unnecessary since checked for 0?
                            .to_string()
                    }
                }
            },
        )),
        10 => Ok(structs::BaseType::UINT8Z(data.into())), // Z = ?
        13 => Ok(structs::BaseType::BYTE(data.into())),   // byte
        11 => Ok(structs::BaseType::UINT16Z({
            let mut buf: Vec<u16> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_u16_into(data, &mut buf),
                1 => BigEndian::read_u16_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        12 => Ok(structs::BaseType::UINT32Z({
            let mut buf: Vec<u32> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_u32_into(data, &mut buf),
                1 => BigEndian::read_u32_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        14 => Ok(structs::BaseType::SINT64({
            let mut buf: Vec<i64> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_i64_into(data, &mut buf),
                1 => BigEndian::read_i64_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        15 => Ok(structs::BaseType::UINT64({
            let mut buf: Vec<u64> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_u64_into(data, &mut buf),
                1 => BigEndian::read_u64_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        16 => Ok(structs::BaseType::UINT64Z({
            let mut buf: Vec<u64> = vec![0; data.len() / base_type_len];
            match architecture {
                0 => LittleEndian::read_u64_into(data, &mut buf),
                1 => BigEndian::read_u64_into(data, &mut buf),
                _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
            }
            buf
        })),
        _ => Err(errors::ParseError::UnknownBaseType(basetype)),
    }
}

/// Parse FIT header
fn header(data: &[u8]) -> Result<structs::FitHeader, errors::ParseError> {
    let crc = match data.len() {
        12 => None,
        14 => Some(LittleEndian::read_u16(&data[12..14])),
        x => return Err(errors::ParseError::UnexpectedHeaderSize(x)),
    };

    Ok(structs::FitHeader {
        headersize: data[0],
        protocol: data[1],
        profile: LittleEndian::read_u16(&data[2..4]),
        datasize: LittleEndian::read_u32(&data[4..8]),
        dotfit: [
            char::from(data[8]),
            char::from(data[9]),
            char::from(data[10]),
            char::from(data[11]),
        ],
        crc, // NOTE: note yet used/calculated, just a u16
    })
}

/// Attempts to parse &[u8] into `DefinitionMessage`.
/// If developer data is present, the relevant field_descriptions are looked up and converted
/// into DefinitionFields.
fn definition_message(
    data: &[u8],
    dev_data: Option<&[u8]>,
    field_descriptions: &HashMap<(u8, u8), structs::FieldDescriptionMessage>,
) -> Result<structs::DefinitionMessage, errors::ParseError> {
    let architecture = data[2]; // 0 -> LE, 1 -> BE
    let global_id = match architecture {
        0 => LittleEndian::read_u16(&data[3..5]),
        1 => BigEndian::read_u16(&data[3..5]),
        _ => return Err(errors::ParseError::InvalidArchitecture(architecture)),
    };

    // GET DEFINITION FIELDS
    let mut definition_fields: Vec<structs::DefinitionField> = Vec::new();
    let mut data_message_length: usize = 1; // header is byte 0
    let def_field_number = usize::from(data[5]);

    for i in (0..def_field_number * 3).step_by(3) {
        // each field: [length, baselength, basetype], all u8
        let field_definition_number = data[i + 6];
        let field = structs::DefinitionField {
            field_definition_number,
            size: data[i + 7],
            base_type: data[i + 8], // use .get()? index out of bounds: the len is 35 but the index is 35
            field_name: messages::field_types::get_fieldtype(global_id, field_definition_number),
            units: None,
            scale: None,  // include from Profile.xslx for normal defs?
            offset: None, // include from Profile.xslx for normal defs?
        };
        data_message_length += usize::from(field.size);
        definition_fields.push(field);
    }

    // GET DEVELOPER FIELDS, GENERATE "NORMAL" DEFINITIONS FROM FIELD DESCRIPTIONS
    let mut developer_fields: Vec<structs::DefinitionField> = Vec::new();
    if let Some(dev) = dev_data {
        // byte 0 = number of dev fields
        let dev_field_number = usize::from(dev[0]);
        for i in (1..dev_field_number * 3).step_by(3) {
            let dev_def = match field_descriptions.get(&(dev[i], dev[i + 2])) {
                Some(fd) => fd,
                None => {
                    return Err(errors::ParseError::UnknownFieldDescription((
                        dev[i],
                        dev[i + 2],
                    )))
                }
            };
            let field = structs::DefinitionField {
                field_definition_number: dev_def.field_definition_number,
                size: dev[i + 1],
                base_type: dev_def.fit_base_type_id,
                field_name: dev_def.field_name.to_owned(),
                units: dev_def.units.to_owned(),
                scale: dev_def.scale,
                offset: dev_def.offset,
            };
            data_message_length += usize::from(field.size);
            developer_fields.push(field);
        }
    }

    Ok(structs::DefinitionMessage {
        header: data[0],
        reserved: data[1],
        architecture,
        global: global_id,
        definition_fields,
        developer_fields,
        data_message_length,
    })
}

/// Attempts to parse &[u8] into `DataMessage`.
/// Checks for and parses developer data into `DataMessage` if present in input Definition.
fn data_message(
    data: &[u8],
    definition: &structs::DefinitionMessage,
    unchecked_string: bool,
) -> Result<structs::DataMessage, errors::ParseError> {
    let mut fields: Vec<structs::DataField> = Vec::new();
    let mut dev_fields: Vec<structs::DataField> = Vec::new();

    let mut index: usize = 1; // header is byte 0

    // NORMAL DATA
    for field in definition.definition_fields.iter() {
        let slice = &data[index..index + usize::from(field.size)];
        let values = get_basevalues(
            field.base_type,
            slice,
            definition.architecture,
            unchecked_string,
        )?;

        index += usize::from(field.size);

        fields.push(structs::DataField {
            field_definition_number: field.field_definition_number,
            description: messages::field_types::get_fieldtype(
                definition.global,
                field.field_definition_number,
            ),
            units: None,
            data: values,
        });
    }

    // DEVELOPER DATA
    for field in definition.developer_fields.iter() {
        let slice = &data[index..index + usize::from(field.size)];
        let values = get_basevalues(
            field.base_type,
            slice,
            definition.architecture,
            unchecked_string,
        )?;

        index += usize::from(field.size);

        dev_fields.push(structs::DataField {
            field_definition_number: field.field_definition_number,
            description: field.field_name.to_owned(),
            units: field.units.to_owned(),
            data: values,
        });
    }

    Ok(structs::DataMessage {
        header: data[0],
        global: definition.global,
        description: messages::message_types::get_messagetype(definition.global),
        fields,
        dev_fields,
    })
}

/// Parses FIT-file. Returns FitData struct containing `Header` struct and data as a HashMap.
/// Key is numerical FIT `global_id` (e.g. `gps_metadata` = 160), value is Vec<DataMessage>.
pub fn parse_fit(
    path: &Path,
    global_id: Option<&u16>, // optionally extract only a specific message type
    debug: bool,             // print definition and data messages while parsing
    debug_unchecked_string: bool, // same as debug but parses strings as unchecked utf-8
) -> Result<structs::FitFile, errors::FitError> {
    let mut fitdata: Vec<structs::DataMessage> = Vec::new();

    let parse_type = match global_id {
        Some(g) => ParseMethod::Filter(*g),
        None => {
            if debug {
                ParseMethod::Debug
            } else {
                ParseMethod::Full
            }
        }
    };

    // MESSAGE/DATA STRUCTURE DEFINITIONS, LOOKUP VIA LOCAL ID (0-15)
    // "normal" definitions
    // Due to the nature of FIT local ID:s this HashMap will only ever hold a maximum of 16 items.
    let mut definitions: HashMap<u8, structs::DefinitionMessage> = HashMap::new();
    // developer data definitions, via field_description_message
    // key: (field_definition_number, developer_data_index) in dev def field idx 0, 2
    let mut developer_definitions: HashMap<(u8, u8), structs::FieldDescriptionMessage> =
        HashMap::new();

    let data = std::fs::read(path)?; // use this instead of read_bytes, fit = small
    if data.is_empty() {
        return Err(errors::FitError::Fatal(errors::ParseError::NoData(
            data.len(),
        )));
    }

    // GET FIT HEADER, READ DATA LOAD
    let header_size: usize = data[0].into();
    let header_data = &data[0..header_size];
    let header = header(header_data)?;

    // partial extraction along with a custom error message
    // so far only for files where specified data size exceeds file size, raising boundary errors
    let mut error_kind: Option<errors::ParseError> = None;

    let data_size: usize = match header.datasize {
        // Estimate data size if 0 in header, occurred in one Garmin VIRB file so far
        0 => {
            let crc_len: u8 = if header.crc.is_some() { 2 } else { 0 };
            let size = data.len() - header_size - crc_len as usize;
            error_kind = Some(errors::ParseError::DataSizeZero(size));
            size
        }

        // Estimate data size if it exceeds file size in header (firmware bug?).
        // See fit bikeroutes from musette.se, all data sizes exceed file size by 11 bytes.
        x if x as usize > data.len() => {
            let crc_len: usize = if header.crc.is_some() { 2 } else { 0 };
            let size = data.len() - header_size - crc_len;
            error_kind = Some(errors::ParseError::DataSizeExceedsFileSize(
                structs::DataSize {
                    read: size as usize,
                    expected: x as usize,
                },
            ));
            size
        }
        _ => header.datasize as usize,
    };

    // MAIN LOOP
    // every increment to index must euqal the length of the current message
    // to ensure new offset is at a message header for each loop
    let mut index: usize = header_size; // global data index/cursor position, start after header

    while index < data_size as usize {
        let message_header = match data[index] {
            255 => {
                return Err(errors::FitError::Partial(
                    errors::ParseError::InvalidMessageHeader((255, index)),
                    structs::FitFile {
                        path: path.to_owned(),
                        header,
                        records: fitdata,
                        crc: None,
                        parse: parse_type,
                    },
                ));
            }
            _ => data[index],
        };

        let local_id = 0b0000_1111 & message_header; // local id = 0-15, 4 least significant bits

        if bit_set(message_header, 6) {
            // FIT DEFINTION MESSAGE (HEADER BIT: X1XXXXXX)

            // derive total length of definition message
            let def_len: usize = 6 + data[index + 5] as usize * 3;
            let dev_len: usize = if bit_set(message_header, 5) {
                // check developer data bit
                1 + data[index + def_len] as usize * 3
            } else {
                0
            };

            let def = &data[index..index + def_len];
            let dev = match dev_len {
                0 => None,
                _ => Some(&data[index + def_len..index + def_len + dev_len]),
            };

            if debug {
                println!("[{}/{}] HEADER: {:#010b}", index, data_size, message_header);
                println!("  DEF U8 {:?}\n  DEV U8 {:?}", def, dev);
            };

            match definition_message(def, dev, &developer_definitions) {
                Ok(msg) => {
                    if debug {
                        println!("  PARSED {:#?}", msg);
                    };
                    definitions.insert(local_id, msg)
                }
                Err(e) => {
                    return Err(errors::FitError::Partial(
                        e,
                        structs::FitFile {
                            path: path.to_owned(),
                            header,
                            records: fitdata,
                            crc: None,
                            parse: parse_type,
                        },
                    ))
                }
            };

            index += def_len + dev_len;
        } else {
            // FIT DATA MESSAGE (HEADER BITS: X0XXXXXX)
            if bit_set(message_header, 7) {
                return Err(errors::FitError::Partial(
                    errors::ParseError::UnsupportedFeature(
                        errors::Feature::CompressedTimestampHeader,
                    ),
                    structs::FitFile {
                        path: path.to_owned(),
                        header,
                        records: fitdata,
                        crc: None,
                        parse: parse_type,
                    },
                ));
            }
            let definition = match definitions.get(&local_id) {
                Some(def) => def,
                None => {
                    return Err(errors::FitError::Partial(
                        errors::ParseError::UnknownDefinition(local_id),
                        structs::FitFile {
                            path: path.to_owned(),
                            header,
                            records: fitdata,
                            crc: None,
                            parse: parse_type,
                        },
                    ));
                }
            };

            // Parse filter, continue before parsing data messages.
            // Intendend for the FitFile.parse_filter(global) method
            // Note that this will ignore developer definitions.
            if let Some(g) = global_id {
                if g != &definition.global {
                    index += definition.data_message_length;
                    continue;
                }
            }

            let slice = &data[index..index + definition.data_message_length];

            if debug {
                println!(
                    "[{}/{}] HEADER: {:#010b}\n  DAT U8 {:?}",
                    index, data_size, message_header, slice
                );
            }

            let message = match data_message(slice, definition, debug_unchecked_string) {
                Ok(msg) => msg,
                Err(e) => {
                    return Err(errors::FitError::Partial(
                        e,
                        structs::FitFile {
                            path: path.to_owned(),
                            header,
                            records: fitdata,
                            crc: None,
                            parse: parse_type,
                        },
                    ))
                }
            };

            if debug {
                println!("  PARSED {:#?}", message);
            }

            if definition.global == 206 {
                match process::field_description_message(&message) {
                    Ok(f) => {
                        developer_definitions
                            .insert((f.field_definition_number, f.developer_data_index), f);
                    }
                    Err(e) => {
                        return Err(errors::FitError::Partial(
                            e,
                            structs::FitFile {
                                path: path.to_owned(),
                                header,
                                records: fitdata,
                                crc: None,
                                parse: parse_type,
                            },
                        ))
                    }
                }
            }

            fitdata.push(message);

            index += definition.data_message_length;
        }
    }

    if debug {
        println!("FINAL INDEX:        {}", index);
        if header.crc.is_some() {
            // crc check not yet implemented
            println!(
                "FINAL CRC (UINT16): [{} {}] -> {}",
                data[index],
                data[index + 1],
                LittleEndian::read_u16(&data[index..=index + 1])
            );
        }
    }

    // Error_kind currently only used to report:
    // - header specifies data size 0
    // - data size > file size
    // Despite these errors it may still be possible to do a full parse
    // if estimating data size.
    // Other non-fatal errors with partial data reads are returned in the loop
    // Note that "non-fatal" in this case means some data could be extracted,
    // but that the error was indeed fatal in terms of not being able to
    // continue the parse process...

    let crc16 = if error_kind.is_some() {
        None // only get crc if no error since FIT file may be truncated
    } else {
        Some(LittleEndian::read_u16(&data[index..=index + 1]))
    };

    let fitfile = structs::FitFile {
        path: path.to_owned(),
        header,
        records: fitdata,
        crc: crc16, // set to crc16/last two bytes of FIT file
        parse: parse_type,
    };

    match error_kind {
        Some(e) => Err(errors::FitError::Partial(e, fitfile)),
        None => Ok(fitfile),
    }
}

/// Get UUID from unaltered VIRB video clip (MP4 or GLV)
pub fn get_video_uuid(path: &Path) -> Result<Option<String>, Mp4Error> {
    let mut video = File::open(path)?;
    let file_size = match video.metadata()?.len() {
        0 => return Err(Mp4Error::UnexpectedFileSize(0).into()),
        l => l,
    };

    let mut index = 0;
    let mut uuid = None;

    let container = ["moov", "udta"]; // virb mp4 hierarchy: moov->udta->uuid

    while index < file_size - 8 {
        let size = BigEndian::read_u32(&read_bytes(&mut video, index, 4)?);
        if size == 0 {
            // Dropbox has 1024 byte placeholders (content all 0:s)
            return Err(Mp4Error::UnexpectedAtomSize(size as u64).into());
        }
        let name = std::str::from_utf8(&read_bytes(&mut video, index + 4, 4)?)?.to_string();
        let ext_size: Option<u64> = match size {
            1 => Some(BigEndian::read_u64(&read_bytes(&mut video, index + 8, 8)?)),
            _ => None,
        };

        if container.contains(&&name[..]) {
            index += 8;
            if index >= file_size - 8 {
                break; // required...?
            };
        } else {
            if name == "uuid" {
                uuid = Some(
                    std::str::from_utf8(&read_bytes(&mut video, index + 8, size as u64 - 8)?)?
                        .trim_matches(char::from(0))
                        .to_string(),
                );
                break;
            }
            index += match ext_size {
                Some(s) => s,
                None => size as u64,
            }
        }
    }
    Ok(uuid)
}

// Get embedded FIT metadata concatenated VIRB video session
// Note: Only looks for keys embedded by GeoELAN.
// pub fn get_video_meta(path: &Path) -> Result<Option<String>, Mp4Error> {
//     let mut video = File::open(path)?;
//     let file_size = match video.metadata()?.len() {
//         0 => return Err(Mp4Error::UnexpectedFileSize(0).into()),
//         l => l,
//     };

//     let mut index = 0;
//     let mut uuid = None;

//     // Atom udta @ 1065504223 of size: 918, ends @ 1065505141
//     //      Atom meta @ 1065504231 of size: 910, ends @ 1065505141
//     //          Atom hdlr @ 1065504243 of size: 33, ends @ 1065504276
//     //          Atom keys @ 1065504276 of size: 129, ends @ 1065504405				 ~
//     //          Atom ilst @ 1065504405 of size: 736, ends @ 1065505141
//     //              Atom  @ 1065504413 of size: 431, ends @ 1065504844
//     //                  Atom data
//     //              ... more data

//     let container = ["moov", "udta", "meta"]; // virb mp4 hierarchy: moov->udta->uuid

//     let mut keys: Vec<String> = Vec::new(); // from "keys" Atom
//     let mut values: Vec<String> = Vec::new(); // from "ilst/data" Atoms, should have same length as keys

//     while index < file_size - 8 {
//         let size = BigEndian::read_u32(&read_bytes(&mut video, index, 4)?);
//         if size == 0 {
//             // Dropbox has 1024 byte placeholders (content all 0:s)
//             return Err(Mp4Error::UnexpectedAtomSize(size as u64).into());
//         }
//         let name = std::str::from_utf8(&read_bytes(&mut video, index + 4, 4)?)?.to_string();
//         let ext_size: Option<u64> = match size {
//             1 => Some(BigEndian::read_u64(&read_bytes(&mut video, index + 8, 8)?)),
//             _ => None,
//         };

//         if container.contains(&&name[..]) {
//             index += 8;
//             if index >= file_size - 8 {
//                 break; // required...?
//             };
//         } else {
//             if name == "uuid" {
//                 uuid = Some(
//                     std::str::from_utf8(&read_bytes(&mut video, index + 8, size as u64 - 8)?)?
//                         .trim_matches(char::from(0))
//                         .to_string(),
//                 );
//                 break;
//             }
//             index += match ext_size {
//                 Some(s) => s,
//                 None => size as u64,
//             }
//         }
//     }
//     Ok(uuid)
// }
