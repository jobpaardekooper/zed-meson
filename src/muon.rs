use std::{
    fs::{self},
    path::PathBuf,
};
use zed_extension_api::{
    self as zed, LanguageServerId, LanguageServerInstallationStatus as LSPStatus, Result,
};

static MUON_VERSION_TAG: &str = "v0.5.0";

use crate::utils::file_exists;

pub fn install_or_find_muon(id: &LanguageServerId) -> Result<String, String> {
    let (platform, arch) = zed::current_platform();

    if arch == zed::Architecture::X86 {
        return Err("Unsupported architecture: x86".to_owned());
    }
    if platform == zed::Os::Windows && arch == zed::Architecture::Aarch64 {
        return Err("Muon does not currently support Windows on Aarch64".to_owned());
    }

    let arch_tag = match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "amd64",
        zed::Architecture::X86 => "amd64",
    };

    let platform_tag = match platform {
        zed::Os::Linux => "linux",
        zed::Os::Windows => "win",
        zed::Os::Mac => "macos",
    };

    let windows_download_postfix = format!("{}-{}.exe", arch_tag, platform_tag);
    let linux_download_postfix = format!("{}-{}", arch_tag, platform_tag);
    let macos_download_postfix = "universal-macos.zip";

    let download_dir_name = format!("muon-{}-{}-{}", MUON_VERSION_TAG, arch_tag, platform_tag);
    let bin_path = format!("{}/muon", download_dir_name);
    let download_file_name = format!(
        "muon-{}-{}",
        MUON_VERSION_TAG,
        match platform {
            zed::Os::Linux => &linux_download_postfix,
            zed::Os::Windows => &windows_download_postfix,
            zed::Os::Mac => macos_download_postfix,
        }
    );

    if !file_exists(&PathBuf::from(&bin_path)) {
        zed::set_language_server_installation_status(&id, &LSPStatus::Downloading);
        let download_url = format!(
            "https://muon.build/releases/{}/{}",
            MUON_VERSION_TAG, download_file_name
        );

        if platform != zed::Os::Mac {
            fs::create_dir(&download_dir_name).map_err(|e| e.to_string())?;
        }

        println!("Downloading Muon from {}", download_url);
        zed::set_language_server_installation_status(&id, &LSPStatus::Downloading);
        zed::download_file(
            &download_url,
            match platform {
                zed::Os::Mac => &download_dir_name,
                _ => &bin_path,
            },
            match platform {
                zed::Os::Mac => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::Uncompressed,
            },
        )?;
    }

    return Ok(bin_path);
}
