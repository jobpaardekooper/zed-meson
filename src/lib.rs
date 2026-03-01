mod mesonlsp;
mod muon;
mod utils;

use zed_extension_api::{
    self as zed, lsp::Completion, CodeLabel, CodeLabelSpan, LanguageServerId,
    LanguageServerInstallationStatus as LSPStatus, Result, Worktree,
};

struct MesonExtension {
    // ... state
}

impl MesonExtension {
    fn lsp_path(&mut self, id: &LanguageServerId, tree: &Worktree) -> Result<String> {
        // Prefer locally installed language servers when available.
        if let Some(path) = tree.which(id.as_ref()) {
            println!("Using local installation at: {}", path);
            return Ok(path);
        }

        println!("No local LSP installation found, downloading...");

        // TODO: Currently, if you update the extension without also opening a meson.build
        // file that will trigger the language server to start, you might later fail to
        // download the latest version of the LSP and we will not try to use the older version
        // even if you did have it installed before
        zed::set_language_server_installation_status(&id, &LSPStatus::CheckingForUpdate);

        let bin_path = match id.as_ref() {
            mesonlsp::LANGUAGE_SERVER_ID => mesonlsp::install_or_find_mesonlsp(&id)?,
            muon::LANGUAGE_SERVER_ID => muon::install_or_find_muon(&id)?,
            _ => return Err(format!("Unsupported language server: {}", id.as_ref())),
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
        let args = match id.as_ref() {
            muon::LANGUAGE_SERVER_ID => {
                vec!["analyze".to_string(), "-l".to_string(), "lsp".to_string()]
            }
            mesonlsp::LANGUAGE_SERVER_ID => vec!["--lsp".to_string()],
            _ => return Err(format!("Unrecognized language server for Meson: {id}")),
        };

        Ok(zed::Command {
            command: self.lsp_path(id, tree)?,
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
