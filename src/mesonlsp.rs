// https://github.com/JCWasmx86/mesonlsp

use std::path::PathBuf;
use zed_extension_api::{
    self as zed, LanguageServerId, LanguageServerInstallationStatus as LSPStatus, Result,
};

use crate::utils::file_exists;

pub const LANGUAGE_SERVER_ID: &str = "mesonlsp";

const MESONLSP_REPOSITORY: &str = "JCWasmx86/mesonlsp";

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

    let release = zed::latest_github_release(
        MESONLSP_REPOSITORY,
        zed::GithubReleaseOptions {
            require_assets: true,
            pre_release: false,
        },
    )?;

    let download_file_name = format!("mesonlsp-{}-{}.zip", arch_tag, platform_tag);
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == download_file_name)
        .ok_or_else(|| {
            format!(
                "MesonLSP release {} has no asset named {}",
                release.version, download_file_name
            )
        })?;

    let download_dir_name = format!("mesonlsp-{}-{}-{}", release.version, arch_tag, platform_tag);
    let bin_path = format!("{}/mesonlsp", download_dir_name);

    if !file_exists(&PathBuf::from(&bin_path)) {
        zed::set_language_server_installation_status(id, &LSPStatus::Downloading);

        println!("Downloading MesonLSP from {}", asset.download_url);
        zed::download_file(
            &asset.download_url,
            &download_dir_name,
            zed::DownloadedFileType::Zip,
        )?;
    }

    Ok(bin_path)
}
