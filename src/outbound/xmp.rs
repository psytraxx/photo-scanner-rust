use std::path::Path;

use crate::domain::ports::FileMeta;
use anyhow::Result;
use async_trait::async_trait;
use tracing::debug;
use xmp_toolkit::{xmp_ns::DC, OpenFileOptions, XmpFile, XmpMeta};

#[derive(Debug, Clone)]
pub struct XMP {}

const XMP_DESCRIPTION: &str = "description";

#[async_trait]
impl FileMeta for XMP {
    async fn write(&self, text: &str, path: &Path) -> Result<()> {
        // Step 1: Open the JPEG file with XmpFile for reading and writing XMP metadata
        let mut xmp_file = XmpFile::new()?;

        xmp_file
            .open_file(
                path,
                OpenFileOptions::default()
                    .only_xmp()
                    .for_update()
                    .use_smart_handler(),
            )
            .or_else(|_| {
                xmp_file.open_file(
                    path,
                    OpenFileOptions::default()
                        .only_xmp()
                        .for_update()
                        .use_packet_scanning(),
                )
            })?;

        // Step 2: Try to extract existing XMP metadata
        let mut xmp = if let Some(existing_xmp) = xmp_file.xmp() {
            debug!("XMP metadata exists. Parsing it...");
            existing_xmp
        } else {
            debug!("No XMP metadata found. Creating a new one.");
            XmpMeta::new()?
        };

        /*  xmp.iter(IterOptions::default()).for_each(|p| {
            debug!("{:?}", p);
        });*/

        xmp.set_localized_text(DC, XMP_DESCRIPTION, None, "x-default", text)?;

        xmp_file.put_xmp(&xmp)?;
        xmp_file.close();

        Ok(())
    }
}
