use byteorder::{ByteOrder, LittleEndian};
use little_exif::{endian::Endian, exif_tag::ExifTag, metadata};

use super::error::MetadataError;

pub fn get_string(exif: &metadata::Metadata, tag: &ExifTag) -> Result<String, MetadataError> {
    Ok(String::from_utf8(
        exif.get_tag(tag)
            .next()
            .ok_or(MetadataError::StringParseError(format!(
                "failed to get string from tag: {:?}",
                tag
            )))?
            .value_as_u8_vec(&Endian::Little),
    )
    .map_err(|_| {
        MetadataError::StringParseError(format!("failed to parse string from tag: {:?}", tag))
    })?
    .replace('\0', ""))
}

pub fn get_u16(exif: &metadata::Metadata, tag: &ExifTag) -> Result<u16, MetadataError> {
    let value = exif
        .get_tag(tag)
        .next()
        .ok_or(MetadataError::IntParseError(format!(
            "failed to get u16 from tag: {:?}",
            tag
        )))?
        .value_as_u8_vec(&Endian::Little);
    if value.len() < 2 {
        return Err(MetadataError::IntParseError(format!(
            "value too short for u16: {:?}",
            tag
        )));
    }
    if value.len() > 2 {
        return Err(MetadataError::IntParseError(format!(
            "value too long for u16: {:?}, len: {}",
            tag,
            value.len()
        )));
    }
    Ok(LittleEndian::read_u16(&value))
}

pub fn get_u32(exif: &metadata::Metadata, tag: &ExifTag) -> Result<u32, MetadataError> {
    let value = exif
        .get_tag(tag)
        .next()
        .ok_or(MetadataError::IntParseError(format!(
            "failed to get u32 from tag: {:?}",
            tag
        )))?
        .value_as_u8_vec(&Endian::Little);
    if value.len() < 4 {
        return Err(MetadataError::IntParseError(format!(
            "value too short for u32: {:?}",
            tag
        )));
    }
    if value.len() > 4 {
        return Err(MetadataError::IntParseError(format!(
            "value too long for u32: {:?}, len: {}",
            tag,
            value.len()
        )));
    }
    Ok(LittleEndian::read_u32(&value))
}

pub fn get_rational_64s(
    exif: &metadata::Metadata,
    tag: &ExifTag,
) -> Result<(i32, i32), MetadataError> {
    let value = exif
        .get_tag(tag)
        .next()
        .ok_or(MetadataError::IntParseError(format!(
            "failed to get rational from tag: {:?}",
            tag
        )))?
        .value_as_u8_vec(&Endian::Little);
    if value.len() < 8 {
        return Err(MetadataError::IntParseError(format!(
            "value too short for rational: {:?}",
            tag
        )));
    }
    if value.len() > 8 {
        return Err(MetadataError::IntParseError(format!(
            "value too long for rational: {:?}, len: {}",
            tag,
            value.len()
        )));
    }
    let numerator = LittleEndian::read_i32(&value[0..4]);
    let denominator = LittleEndian::read_i32(&value[4..8]);
    Ok((numerator, denominator))
}

pub fn get_rational_64s_as_string(
    exif: &metadata::Metadata,
    tag: &ExifTag,
) -> Result<String, MetadataError> {
    let (numerator, denominator) = get_rational_64s(exif, tag)?;
    Ok(format!("{}/{}", numerator, denominator))
}

pub fn get_rational_64s_as_float(
    exif: &metadata::Metadata,
    tag: &ExifTag,
) -> Result<f64, MetadataError> {
    let (numerator, denominator) = get_rational_64s(exif, tag)?;
    if denominator == 0 {
        return Err(MetadataError::IntParseError(format!(
            "denominator is zero for rational: {:?}",
            tag
        )));
    }
    Ok(numerator as f64 / denominator as f64)
}
