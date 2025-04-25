use crate::exif_data::error::MetadataError;
use little_exif::{endian::Endian, exif_tag::ExifTag, metadata};
use std::{fmt::Display, path::Path};
use xmp_toolkit::{xmp_ns, OpenFileOptions, XmpFile};

use super::read_exif::get_string;

/// This parses metadata that will not be edited from the exif data of a photo.
#[derive(Debug)]
pub struct EditableMetadata {
    rating: u16,
    tags: String,
}

impl EditableMetadata {
    pub fn from_exif(exif: &metadata::Metadata, path: &Path) -> Result<Self, MetadataError> {
        let endian = exif.get_endian();
        if endian != Endian::Little {
            return Err(MetadataError::UnsupportedBigEndian);
        }

        let mut f = XmpFile::new().map_err(|_| {
            MetadataError::XmpError(format!(
                "Failed to create XmpFile for path: {}",
                path.display()
            ))
        })?;
        f.open_file(
            path,
            OpenFileOptions::default().only_xmp().use_smart_handler(),
        )
        .map_err(|_| {
            MetadataError::XmpError(format!(
                "Failed to open XmpFile for path: {}",
                path.display()
            ))
        })?;

        let xmp = f.xmp().ok_or(MetadataError::XmpError(format!(
            "Failed to get XMP data for path: {}",
            path.display()
        )))?;

        let rating = xmp
            .property_i32(xmp_ns::XMP, "Rating")
            .ok_or(MetadataError::XmpError(format!(
                "Failed to get XMP rating for path: {}",
                path.display()
            )))?
            .value as u16;

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
