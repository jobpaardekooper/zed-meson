// https://github.com/JCWasmx86/mesonlsp

use std::path::PathBuf;
use zed_extension_api::{
    self as zed, LanguageServerId, LanguageServerInstallationStatus as LSPStatus, Result,
};

use crate::utils::file_exists;

static MESONLSP_VERSION_TAG: &str = "v4.3.7";

// TODO: Reuse more logic between this and the muon installer
pub fn install_or_find_mesonlsp(id: &LanguageServerId) -> Result<String, String> {
    let (platform, arch) = zed::current_platform();

    if arch == zed::Architecture::X86 {
        return Err("Unsupported architecture: x86".to_owned());
    }
    if platform == zed::Os::Windows && arch == zed::Architecture::Aarch64 {
        return Err("mesonlsp does not currently support Windows on Aarch64".to_owned());
    }

    let arch_tag = match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "x86_64",
        zed::Architecture::X86 => "x86_64",
    };

    let platform_tag = match platform {
        zed::Os::Linux => "unknown-linux-musl",
        zed::Os::Windows => "pc-windows-gnu",
        zed::Os::Mac => "apple-darwin",
    };

    let download_dir_name = format!(
        "mesonlsp-{}-{}-{}",
        MESONLSP_VERSION_TAG, arch_tag, platform_tag,
    );
    let bin_path = format!("{}/mesonlsp", download_dir_name);
    let download_file_name = format!("mesonlsp-{}-{}.zip", arch_tag, platform_tag);

    if !file_exists(&PathBuf::from(&bin_path)) {
        zed::set_language_server_installation_status(&id, &LSPStatus::Downloading);
        let download_url = format!(
            "https://github.com/JCWasmx86/mesonlsp/releases/download/{}/{}",
            MESONLSP_VERSION_TAG, download_file_name
        );

        println!("Downloading mesonlsp from {}", download_url);
        zed::set_language_server_installation_status(&id, &LSPStatus::Downloading);
        zed::download_file(
            &download_url,
            &download_dir_name,
            zed::DownloadedFileType::Zip,
        )?;
    }

    return Ok(bin_path);
}
