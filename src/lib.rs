use zed_extension_api as zed;

struct NotedExtension;

impl zed::Extension for NotedExtension {
    fn new() -> Self {
        NotedExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // 1. Check NOTED_LSP_PATH in the shell environment (set in ~/.zshrc or ~/.bashrc).
        //    worktree.shell_env() reads the actual user shell env, unlike std::env::var
        //    which doesn't work inside the Zed WASM sandbox.
        // 2. Fall back to searching PATH via worktree.which().
        // Phase 4 will replace this with auto-download from GitHub Releases.
        let command = worktree
            .shell_env()
            .into_iter()
            .find(|(k, _)| k == "NOTED_LSP_PATH")
            .map(|(_, v)| v)
            .or_else(|| worktree.which("noted-lsp"))
            .ok_or("noted-lsp not found. Set NOTED_LSP_PATH in your shell profile.")?;

        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NotedExtension);
