use crate::Archive;
use std::fmt::Formatter;

impl<R> core::fmt::Debug for Archive<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "File version: {:?}", self.file_version)?;
        writeln!(f, "File version UE5: {:?}", self.file_version_ue5)?;
        writeln!(f, "Legacy version: {:?}", self.legacy_version)?;
        writeln!(f, "File licensee version: {:?}", self.file_licensee_version)?;
        writeln!(f, "With editoronly data: {:?}", self.with_editoronly_data)?;
        Ok(())
    }
}
