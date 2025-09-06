use crate::serialization::UnrealEngineVersion;

impl std::fmt::Debug for UnrealEngineVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "UE_{}.{}.{}-{}.{}-{}",
            self.major,
            self.minor,
            self.patch,
            &self.branch_name,
            self.changelist,
            self.is_licensee_version,
        )?;

        Ok(())
    }
}
