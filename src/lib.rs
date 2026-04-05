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
        Err("LSP not configured yet".into())
    }
}

zed::register_extension!(NotedExtension);
