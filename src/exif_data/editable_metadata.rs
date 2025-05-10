use crate::exif_data::error::MetadataError;
use little_exif::{endian::Endian, exif_tag::ExifTag, metadata};
use std::fmt::Display;

use super::{
    read_exif::get_string,
    read_xmp::{Xmp, XmpProperties},
};

/// This parses metadata that will not be edited from the exif data of a photo.
#[derive(Debug)]
pub struct EditableMetadata {
    rating: u16,
    tags: String,
}

impl EditableMetadata {
    pub fn from_exif(exif: &metadata::Metadata, xmp: &Xmp) -> Result<Self, MetadataError> {
        let endian = exif.get_endian();
        if endian != Endian::Little {
            return Err(MetadataError::UnsupportedBigEndian);
        }

        let rating = xmp.get_i32(XmpProperties::Rating)? as u16;

        let tags = get_string(exif, &ExifTag::UserComment(Vec::new()))?.replace("ASCII", "");

        Ok(Self { rating, tags })
    }
}

impl Display for EditableMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rating: {}", self.rating)?;
        writeln!(f, "Tags: {}", self.tags)?;
        Ok(())
    }
}
