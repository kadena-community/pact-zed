use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, lsp::CompletionKind, CodeLabelSpan, Result};

struct PactExtension {
    cached_binary_path: Option<String>,
}

#[derive(Clone)]
struct PactBinary {
    path: String,
    environment: Option<Vec<(String, String)>>,
}

impl PactExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<PactBinary> {
        // if let Some(path) = worktree.which("pact") {
        //     let environment = worktree.shell_env();
        //     return Ok(PactBinary {
        //         path,
        //         environment: Some(environment),
        //     });
        // }

        if let Some(path) = &self.cached_binary_path {
            println!("cached path: {:?}", path);
            if fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                return Ok(PactBinary {
                    path: path.clone(),
                    environment: None,
                });
            }
        }
        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::github_release_by_tag_name("kadena-io/pact-5", "development-latest")?;

        let (platform, arch) = zed::current_platform();
        let asset_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::X8664) => {
                "pact-binary-bundle.macos-latest".to_string()
            }
            (zed::Os::Mac, zed::Architecture::Aarch64) => "pact-binary-bundle.macos-m1".to_string(),
            (zed::Os::Linux, zed::Architecture::X8664) => {
                "pact-binary-bundle.ubuntu-latest".to_string()
            }
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
        let mut binary_path = None;
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
        }
        let entries =
            fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
            if entry.file_name().to_str() != Some(&version_dir) {
                fs::remove_dir_all(&entry.path()).ok();
            } else {
                let nix_store_path = entry.path().join("nix/store");
                let entries = nix_store_path
                    .read_dir()
                    .map_err(|e| format!("failed to list nix store directory {e}"))?;
                for entry in entries {
                    let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                    binary_path = Some(entry.path().join("bin/pact").to_str().unwrap().to_string());
                }
            }
        }
        let binary_path = binary_path.ok_or_else(|| {
            let message = format!("failed to find pact binary",);
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Failed(message.clone()),
            );
            message
        })?;
        self.cached_binary_path = Some(binary_path.clone());
        Ok(PactBinary {
            path: binary_path,
            environment: None,
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
            args: vec!["--lsp".to_string()],
            env: pact_binary.environment.unwrap_or_default(),
        })
    }
}

zed::register_extension!(PactExtension);
