// https://muon.build/

use std::{
    fs::{self},
    path::PathBuf,
};
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod, HttpRequest, RedirectPolicy},
    LanguageServerId, LanguageServerInstallationStatus as LSPStatus, Result,
};

use crate::utils::file_exists;

pub const LANGUAGE_SERVER_ID: &str = "muon";

const MUON_RELEASES_URL: &str = "https://muon.build/releases/";

fn latest_release_version(body: &[u8]) -> Result<String> {
    let releases: zed::serde_json::Map<String, zed::serde_json::Value> =
        zed::serde_json::from_slice(body)
            .map_err(|e| format!("Failed to parse Muon releases: {e}"))?;

    // The index contains unordered version directories, including `edge` and
    // development builds. We only accept stable vMAJOR.MINOR.PATCH directory names.
    releases
        .keys()
        .filter_map(|tag| {
            let mut parts = tag.strip_prefix('v')?.split('.');
            let mut version = [0_u64; 3];
            for part in &mut version {
                let digits = parts.next()?;
                if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                *part = digits.parse().ok()?;
            }
            if parts.next().is_some() {
                return None;
            }
            Some((version, tag))
        })
        .max_by_key(|(version, _)| *version)
        .map(|(_, tag)| tag.clone())
        .ok_or_else(|| "No stable Muon release found".to_owned())
}

pub fn install_or_find_muon(id: &LanguageServerId) -> Result<String, String> {
    let (platform, arch) = zed::current_platform();

    if arch == zed::Architecture::X86 {
        return Err("Unsupported architecture: x86".to_owned());
    }
    if platform == zed::Os::Windows && arch == zed::Architecture::Aarch64 {
        return Err("Muon does not currently support Windows on Aarch64".to_owned());
    }

    let response = HttpRequest::builder()
        .method(HttpMethod::Get)
        .url(MUON_RELEASES_URL)
        .header("Accept", "application/json")
        .redirect_policy(RedirectPolicy::FollowLimit(5))
        .build()?
        .fetch()
        .map_err(|e| format!("Failed to fetch Muon releases: {e}"))?;
    let version = latest_release_version(&response.body)?;

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

    let download_dir_name = format!("muon-{}-{}-{}", version, arch_tag, platform_tag);
    let bin_path = format!("{}/muon", download_dir_name);
    let download_file_name = format!(
        "muon-{}-{}",
        version,
        match platform {
            zed::Os::Linux => &linux_download_postfix,
            zed::Os::Windows => &windows_download_postfix,
            zed::Os::Mac => macos_download_postfix,
        }
    );

    if !file_exists(&PathBuf::from(&bin_path)) {
        zed::set_language_server_installation_status(id, &LSPStatus::Downloading);
        let download_url = format!("{}{}/{}", MUON_RELEASES_URL, version, download_file_name);

        if platform != zed::Os::Mac {
            fs::create_dir_all(&download_dir_name).map_err(|e| e.to_string())?;
        }

        println!("Downloading Muon from {}", download_url);
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

    Ok(bin_path)
}
