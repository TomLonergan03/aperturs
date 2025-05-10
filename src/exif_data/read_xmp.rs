use std::path::Path;

use xmp_toolkit::{xmp_ns, OpenFileOptions, XmpFile, XmpMeta, XmpProperty};

use super::error::MetadataError;

pub enum XmpProperties {
    Rating,
}

impl XmpProperties {
    pub fn namespace(&self) -> &'static str {
        match self {
            XmpProperties::Rating => xmp_ns::XMP,
        }
    }

    pub fn property(&self) -> &'static str {
        match self {
            XmpProperties::Rating => "Rating",
        }
    }
}

pub struct Xmp {
    pub xmp_meta: XmpMeta,
}

impl Xmp {
    pub fn read_xmp(path: &Path) -> Result<Self, MetadataError> {
        let mut f = XmpFile::new().map_err(|_| {
            MetadataError::XmpError(format!("Failed to create XmpFile for path: {:?}", path))
        })?;
        f.open_file(
            path,
            OpenFileOptions::default().only_xmp().use_smart_handler(),
        )
        .map_err(|_| {
            MetadataError::XmpError(format!("Failed to open XmpFile for path: {:?}", path))
        })?;

        let xmp = f.xmp().ok_or(MetadataError::XmpError(format!(
            "Failed to get XMP data for path: {:?}",
            path
        )))?;

        Ok(Self { xmp_meta: xmp })
    }

    pub fn get_i32(&self, property: XmpProperties) -> Result<i32, MetadataError> {
        let namespace = property.namespace();
        let prop = property.property();

        Ok(self
            .xmp_meta
            .property_i32(namespace, prop)
            .ok_or(MetadataError::XmpError(format!(
                "Failed to get string from XMP for namespace: {}, property: {}",
                namespace, prop
            )))?
            .value)
    }
}
