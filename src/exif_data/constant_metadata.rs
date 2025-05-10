use super::read_exif;
use crate::exif_data::error::MetadataError;
use little_exif::{endian::Endian, exif_tag::ExifTag, metadata};
use read_exif::{
    get_rational_64s_as_float, get_rational_64s_as_string, get_string, get_u16, get_u32,
};
use std::fmt::Display;

/// This parses metadata that will not be edited from the exif data of a photo.
#[derive(Debug)]
pub struct ConstantMetadata {
    model: String,
    width: u32,
    height: u32,
    iso: u16,
    aperture: String,
    focal_length: f64,
    exposure_time: String,
    time: String,
}

impl ConstantMetadata {
    pub fn from_exif(exif: &metadata::Metadata) -> Result<Self, MetadataError> {
        let endian = exif.get_endian();
        if endian != Endian::Little {
            return Err(MetadataError::UnsupportedBigEndian);
        }
        let width = get_u32(exif, &ExifTag::ExifImageWidth(Vec::new()))?;

        let height = get_u32(exif, &ExifTag::ExifImageHeight(Vec::new()))?;

        let model = get_string(exif, &ExifTag::Model(String::new()))?;

        let iso = get_u16(exif, &ExifTag::ISO(Vec::new()))?;

        let aperture = get_rational_64s_as_string(exif, &ExifTag::FNumber(Vec::new()))?;

        let focal_length = get_rational_64s_as_float(exif, &ExifTag::FocalLength(Vec::new()))?;

        let exposure_time = get_rational_64s_as_string(exif, &ExifTag::ExposureTime(Vec::new()))?;

        let time = get_string(exif, &ExifTag::DateTimeOriginal(String::new()))?;

        Ok(Self {
            width,
            height,
            model,
            iso,
            aperture,
            focal_length,
            exposure_time,
            time,
        })
    }
}

impl Display for ConstantMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Width: {}", self.width)?;
        writeln!(f, "Height: {}", self.height)?;
        writeln!(f, "Model: {}", self.model)?;
        // writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "ISO: {}", self.iso)?;
        writeln!(f, "Aperture: {}", self.aperture)?;
        writeln!(f, "Focal Length: {}mm", self.focal_length)?;
        writeln!(f, "Exposure Time: {}", self.exposure_time)?;
        writeln!(f, "Time: {}s", self.time)?;
        Ok(())
    }
}
