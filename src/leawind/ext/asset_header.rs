use crate::AssetHeader;
use crate::leawind::utils::indent::Indent;
use std::path::Path;

impl<R> AssetHeader<R> {
    fn fmt_imports(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Imports:")?;

        for (i, x) in self.imports.iter().enumerate() {
            writeln!(f, "[{}]", i)?;
            writeln!(f, " outer: ({}) {:?}", x.outer_index, x.outer())?;
            writeln!(f, " import_optional: {}", x.import_optional)?;

            writeln!(
                f,
                " class_package: {:?}",
                self.resolve_name(&x.class_package)
            )?;
            writeln!(f, " class_name: {:?}", self.resolve_name(&x.class_name))?;
            writeln!(
                f,
                " package_name: {:?}",
                x.package_name.map(|x| self.resolve_name(&x))
            )?;
            writeln!(f, " object_name: {:?}", self.resolve_name(&x.object_name))?;
        }

        writeln!(f)?;
        Ok(())
    }

    fn fmt_exports(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Exports:")?;
        for (i, x) in self.exports.iter().enumerate() {
            writeln!(f, "[{}]", i)?;

            writeln!(f, " object_name: {:?}", self.resolve_name(&x.object_name))?;
            writeln!(f, " is_asset: {}", x.is_asset)?;
            writeln!(f, " object_flags: {:08b}", x.object_flags)?;

            writeln!(f, " outer: ({}) {:?}", x.outer_index, x.outer())?;
            writeln!(f, " class: ({}) {:?}", x.class_index, x.class())?;
            writeln!(f, " super: ({}) {:?}", x.super_index, x.superclass())?;
            writeln!(f, " template: ({}) {:?}", x.template_index, x.template())?;

            writeln!(f, " serial_size: {}", x.serial_size)?;
            writeln!(f, " serial_offset: {}", x.serial_offset)?;

            writeln!(f)?;
        }
        Ok(())
    }
}

impl<R> core::fmt::Debug for AssetHeader<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Archive: \n{:?}", self.archive.indent(" "))?;

        writeln!(f, "total_header_size: {}", self.total_header_size)?;
        writeln!(f, "package_name: {}", self.package_name)?;
        writeln!(f, "package_flags: {:0b}", self.package_flags)?;
        writeln!(f, "compression_flags: {:0b}", self.compression_flags)?;
        writeln!(f, "localization_id: {:?}", self.localization_id)?;
        writeln!(f, "engine_version: {:?}", self.engine_version)?;
        writeln!(f, "package_source: {}", self.package_source)?;
        writeln!(
            f,
            "asset_registry_data_offset: {}",
            self.asset_registry_data_offset
        )?;
        writeln!(f, "bulk_data_start_offset: {}", self.bulk_data_start_offset)?;
        writeln!(
            f,
            "names_referenced_from_export_data_count: {}",
            self.names_referenced_from_export_data_count
        )?;

        writeln!(
            f,
            "soft_object_paths_count: {}",
            self.soft_object_paths_count
        )?;
        writeln!(
            f,
            "soft_object_paths_offset: {}",
            self.soft_object_paths_offset
        )?;

        writeln!(
            f,
            "gatherable_text_data_count: {}",
            self.gatherable_text_data_count
        )?;
        writeln!(
            f,
            "gatherable_text_data_offset: {}",
            self.gatherable_text_data_offset
        )?;

        writeln!(f, "names({}): {:?}", self.names.len(), self.names)?;

        self.fmt_imports(f)?;
        self.fmt_exports(f)?;

        Ok(())
    }
}

impl<R> AssetHeader<R> {
    pub fn export(&self, _output_dir: &Path) -> Result<(), crate::error::Error> {
        todo!();
    }
}
