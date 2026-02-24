mod muon;
mod settings;
mod utils;

use crate::settings::get_settings;
use zed_extension_api::{
    self as zed, LanguageServerId, LanguageServerInstallationStatus as LSPStatus, Result, Worktree,
};

struct MesonExtension {
    // ... state
}

impl MesonExtension {
    fn lsp_path(&mut self, id: &LanguageServerId, tree: &Worktree) -> Result<String> {
        // TODO: Write in readme paths for settings: https://zed.dev/docs/extensions/installing-extensions
        let settings = get_settings()?;

        // Use local muon if available and if the user has enabled the setting to look for it in the PATH
        if settings.search_in_path {
            if let Some(path) = tree.which("muon") {
                println!("Using local Muon installation at {}", path);
                return Ok(path);
            }
        }

        println!("No local Muon installation found, downloading...");

        // TODO: Currently, if you update the extension without also opening a meson.build
        // file that will trigger the language server to start, you might later fail to
        // download the latest version of Muon and we will not try to use the older version
        // even if you did have it installed before
        zed::set_language_server_installation_status(&id, &LSPStatus::CheckingForUpdate);

        let bin_path = muon::install_or_find_muon(&id)?;

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
