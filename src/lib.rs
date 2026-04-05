use zed_extension_api as zed;

struct NotedExtension;

impl zed::Extension for NotedExtension {
    fn new() -> Self {
        NotedExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // In dev mode, set NOTED_LSP_PATH to the compiled binary:
        //   export NOTED_LSP_PATH=/path/to/noted/target/debug/noted-lsp
        // Phase 4 will replace this with auto-download from GitHub Releases.
        let command = std::env::var("NOTED_LSP_PATH")
            .unwrap_or_else(|_| "noted-lsp".to_string());

        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NotedExtension);
