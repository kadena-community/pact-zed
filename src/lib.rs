use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

struct PactExtension {
    cached_binary_path: Option<String>,
}

#[derive(Clone)]
struct PactBinary {
    path: String,
    args: Option<Vec<String>>,
    environment: Option<Vec<(String, String)>>,
}

impl PactExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<PactBinary> {
        let mut args: Option<Vec<String>> = None;
        let (platform, _) = zed::current_platform();
        let environment = match platform {
            zed::Os::Mac | zed::Os::Linux => Some(worktree.shell_env()),
            zed::Os::Windows => None,
        };

        if let Ok(lsp_settings) = LspSettings::for_worktree("pact", worktree) {
            if let Some(binary) = lsp_settings.binary {
                args = binary.arguments;
                if let Some(path) = binary.path {
                    return Ok(PactBinary {
                        path: path.clone(),
                        args,
                        environment,
                    });
                }
            }
        }

        if let Some(path) = worktree.which("pact") {
            return Ok(PactBinary {
                path,
                args,
                environment,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            println!("cached path: {:?}", path);
            if fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                return Ok(PactBinary {
                    path: path.clone(),
                    args,
                    environment,
                });
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::github_release_by_tag_name("kadena-io/pact-5", "nightly")?;

        let (platform, arch) = zed::current_platform();
        let asset_name = match (platform, arch) {
            // (zed::Os::Mac, zed::Architecture::X8664) => {
            //     "pact-binary-bundle.macos-latest".to_string()
            // }
            (zed::Os::Mac, zed::Architecture::Aarch64) => "pact-nightly-darwin-aarch64".to_string(),
            (zed::Os::Linux, zed::Architecture::X8664) => "pact-nightly-linux-x64".to_string(),
            _ => {
                return Err(format!(
                    "unsupported platform and architecture combination: {:?} {:?}",
                    platform, arch
                ));
            }
        };

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name.contains(&asset_name))
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("pact-{}", release.version);
        let binary_path = format!("{version_dir}/pact");
        if !fs::metadata(&version_dir).map_or(false, |stat| stat.is_dir()) {
            println!("downloading pact binary");
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(PactBinary {
            path: binary_path,
            args,
            environment,
        })
    }
}

impl zed::Extension for PactExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let pact_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: pact_binary.path,
            args: pact_binary.args.unwrap_or(vec!["--lsp".to_string()]),
            env: pact_binary.environment.unwrap_or_default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("pact", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(PactExtension);
