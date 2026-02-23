mod settings;

use crate::settings::get_settings;
use std::{
    fs::{self},
    path::PathBuf,
};
use zed_extension_api::{
    self as zed, LanguageServerId, LanguageServerInstallationStatus as LSPStatus, Result, Worktree,
};

static MUON_VERSION_TAG: &str = "v0.5.0";

struct MesonExtension {
    // ... state
}

fn file_exists(path: &PathBuf) -> bool {
    fs::metadata(path).map_or(false, |s| s.is_file())
}

impl MesonExtension {
    fn lsp_path(&mut self, id: &LanguageServerId, tree: &Worktree) -> Result<String> {
        // TODO: Write in readme paths for settings: https://zed.dev/docs/extensions/installing-extensions
        let settings = get_settings()?;

        println!("Current settings: {:?}", settings);

        // Use local muon if available
        if let Some(path) = tree.which("muon") {
            println!("Using local Muon installation at {}", path);
            return Ok(path);
        }

        println!("No local Muon installation found, downloading...");

        // TODO: Currently, if you update the extension without also opening a meson.build
        // file that will trigger the language server to start, you might later fail to
        // download the latest version of Muon and we will not try to use the older version
        // even if you did have it installed before
        zed::set_language_server_installation_status(&id, &LSPStatus::CheckingForUpdate);

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

        zed::make_file_executable(&bin_path)?;
        zed::set_language_server_installation_status(&id, &LSPStatus::None);
        return Ok(bin_path);
    }
}

impl zed::Extension for MesonExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        tree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.lsp_path(id, tree)?,
            args: vec!["analyze".to_string(), "-l".to_string(), "lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(MesonExtension);
