use std::fs;
use zed_extension_api as zed;

struct NotedExtension {
    cached_binary_path: Option<String>,
}

const GITHUB_REPO: &str = "sergeevalera/noted";
const LSP_BIN_NAME: &str = "noted-lsp";

impl NotedExtension {
    /// Resolve the LSP binary path, in order of priority:
    /// 1. NOTED_LSP_PATH environment variable (dev mode)
    /// 2. `noted-lsp` on PATH
    /// 3. Previously downloaded binary (cached)
    /// 4. Download from GitHub Releases
    fn resolve_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String, String> {
        // 1. Dev override via env var
        if let Some(path) = worktree
            .shell_env()
            .into_iter()
            .find(|(k, _)| k == "NOTED_LSP_PATH")
            .map(|(_, v)| v)
        {
            return Ok(path);
        }

        // 2. Binary on PATH
        if let Some(path) = worktree.which(LSP_BIN_NAME) {
            return Ok(path);
        }

        // 3. Cached download
        if let Some(ref path) = self.cached_binary_path {
            if fs::metadata(path).is_ok() {
                return Ok(path.clone());
            }
        }

        // 4. Download from GitHub Releases
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("failed to fetch latest release: {e}"))?;

        let (platform, arch) = zed::current_platform();
        let target = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "aarch64-apple-darwin",
            (zed::Os::Mac, zed::Architecture::X8664) => "x86_64-apple-darwin",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
            (zed::Os::Linux, zed::Architecture::X8664) => "x86_64-unknown-linux-gnu",
            (zed::Os::Windows, zed::Architecture::X8664) => "x86_64-pc-windows-msvc",
            _ => return Err(format!("unsupported platform: {platform:?} {arch:?}")),
        };

        let asset_name = format!("{LSP_BIN_NAME}-{target}.tar.gz");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset found for {target} in release {}", release.version))?;

        let version_dir = format!("{LSP_BIN_NAME}-{}", release.version);
        let binary_path = format!("{version_dir}/{LSP_BIN_NAME}");

        if fs::metadata(&binary_path).is_err() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download {}: {e}", asset.name))?;

            zed::make_file_executable(&binary_path)
                .map_err(|e| format!("failed to make binary executable: {e}"))?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for NotedExtension {
    fn new() -> Self {
        NotedExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = self.resolve_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NotedExtension);
