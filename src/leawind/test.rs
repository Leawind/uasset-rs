use crate::AssetHeader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const FILE_LOTUS: &str = r#"D:\TempWork\160-unpack-mb\output\gfp-unpacked-all-quickbms\Saved\ShadowTrackerExtra\Content\Actor_Timeliness\CG032\CG032_Nezha\Arts_Player\Mesh\SK_VH_Lotus.uasset"#;
const FILE_IPAB: &str = r#"D:\TempWork\160-unpack-mb\output\gfp-unpacked-all-quickbms\Saved\ShadowTrackerExtra\Content\Arts_Commerce\2025Q3\IPAB\Avatar_Mesh\Mesh\IPAB_AT_Jacket_1562.uasset"#;
const FILE_IPAB_HD: &str = r#"D:\TempWork\160-unpack-mb\output\gfp-unpacked-all-quickbms\Saved\ShadowTrackerExtra\Content\Arts_Commerce\2025Q3\IPAB\HD\Avatar_Mesh\Mesh\IPAB_AT_Jacket_1562_HD.uasset"#;
const FILE_IPNZ_LV1: &str = r#"D:\TempWork\160-unpack-mb\output\gfp-unpacked-all-quickbms\Saved\ShadowTrackerExtra\Content\Arts_Commerce\2025Q3\IPNZ\Avatar_Mesh\Mesh\IPNZ_AT_Jacket_1561_Lv1.uasset"#;
const FILE_IPNZ_LV2: &str = r#"D:\TempWork\160-unpack-mb\output\gfp-unpacked-all-quickbms\Saved\ShadowTrackerExtra\Content\Arts_Commerce\2025Q3\IPNZ\Avatar_Mesh\Mesh\IPNZ_AT_Jacket_1561_Lv2.uasset"#;
const FILE_IPNZ_HD_LV1: &str = r#"D:\TempWork\160-unpack-mb\output\gfp-unpacked-all-quickbms\Saved\ShadowTrackerExtra\Content\Arts_Commerce\2025Q3\IPNZ\HD\Avatar_Mesh\Mesh\IPNZ_AT_Jacket_1561_Lv1_HD.uasset"#;
const FILE_IPNZ_HD_LV2: &str = r#"D:\TempWork\160-unpack-mb\output\gfp-unpacked-all-quickbms\Saved\ShadowTrackerExtra\Content\Arts_Commerce\2025Q3\IPNZ\HD\Avatar_Mesh\Mesh\IPNZ_AT_Jacket_1561_Lv2_HD.uasset"#;
const FILE_TEST: &str = r#"D:\Workspace\FromGithub\Leawind\uasset-rs\assets\UE418\SimpleRefs\SimpleRefsGraphRef.uasset"#;

/// # Format
///
/// - uasset 文件的大小等于 `sesrial_offset`
/// - uexp 文件大小等于 `serial_size + 4`
/// - uasset大小 + uexp大小 = bulk_data_start_offset + 4
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let uasset_path: &Path = FILE_IPNZ_HD_LV1.as_ref();
    let uasset_file = File::open(uasset_path)?;

    let header = AssetHeader::new(BufReader::new(uasset_file))?;

    println!("Header:\n{:?}", header);
    let uexp_path: &Path = &uasset_path.with_extension("uexp");

    if uexp_path.is_file() {
        let uexp_file = File::open(uexp_path)?;
        println!("Uexp:\n{:?}", uexp_file);
    }
    Ok(())
}
