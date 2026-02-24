mod mesonlsp;
mod muon;
mod settings;
mod utils;

use crate::settings::{get_settings, LspVariant, Settings};
use zed_extension_api::{
    self as zed, lsp::Completion, CodeLabel, CodeLabelSpan, LanguageServerId,
    LanguageServerInstallationStatus as LSPStatus, Result, Worktree,
};

struct MesonExtension {
    // ... state
}

impl MesonExtension {
    fn lsp_path(
        &mut self,
        id: &LanguageServerId,
        tree: &Worktree,
        settings: &Settings,
    ) -> Result<String> {
        // Use locally installed LSP if available and if the user has enabled the setting to look for it in the PATH
        if settings.search_in_path {
            if let Some(path) = match settings.lsp {
                LspVariant::Mesonlsp => tree.which("mesonlsp"),
                LspVariant::Muon => tree.which("muon"),
            } {
                println!("Using local installation at: {}", path);
                return Ok(path);
            }
        }

        println!("No local LSP installation found, downloading...");

        // TODO: Currently, if you update the extension without also opening a meson.build
        // file that will trigger the language server to start, you might later fail to
        // download the latest version of the LSP and we will not try to use the older version
        // even if you did have it installed before
        zed::set_language_server_installation_status(&id, &LSPStatus::CheckingForUpdate);

        let bin_path = match settings.lsp {
            LspVariant::Mesonlsp => mesonlsp::install_or_find_mesonlsp(&id)?,
            LspVariant::Muon => muon::install_or_find_muon(&id)?,
        };

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
        // TODO: Write in readme paths for settings: https://zed.dev/docs/extensions/installing-extensions
        let settings = get_settings()?;

        let args = match settings.lsp {
            LspVariant::Muon => vec!["analyze".to_string(), "-l".to_string(), "lsp".to_string()],
            LspVariant::Mesonlsp => vec!["--lsp".to_string()],
        };

        Ok(zed::Command {
            command: self.lsp_path(id, tree, &settings)?,
            args,
            env: Default::default(),
        })
    }

    // Improve how autocomplete lables are displayed (improves the syntax highlighting in the labels for both mesonlsp and muon)
    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        let kind = match completion.kind {
            Some(zed::lsp::CompletionKind::Function) | Some(zed::lsp::CompletionKind::Method) => {
                match completion.detail {
                    Some(a) => a,
                    _ => completion.label,
                }
            }
            _ => match completion.detail {
                Some(a) => format!("{} {}", completion.label, a),
                _ => completion.label,
            },
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..kind.len())],
            filter_range: (0..kind.len()).into(),
            code: kind,
        })
    }
}

zed::register_extension!(MesonExtension);
