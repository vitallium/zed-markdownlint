use std::{env, fs};

use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

const SERVER_PATH: &str = "node_modules/markdownlint-lsp/lib/index.mjs";
const PACKAGE_NAME: &str = "markdownlint-lsp";

struct MarkdownlintExtension {
    did_find_server: bool,
}

impl MarkdownlintExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                                        "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                                    ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for MarkdownlintExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let lsp_settings =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree).unwrap_or_default();
        let binary_settings = lsp_settings.binary;
        let arguments = binary_settings
            .as_ref()
            .and_then(|binary| binary.arguments.clone());
        let binary_env = binary_settings
            .as_ref()
            .and_then(|binary| binary.env.clone())
            .map(|env| env.into_iter().collect());
        let path = binary_settings
            .as_ref()
            .and_then(|binary| binary.path.clone())
            .or_else(|| worktree.which("markdownlint-lsp-server"));
        if let Some(path) = path {
            return Ok(zed::Command {
                command: path,
                args: arguments.unwrap_or_else(|| vec!["--stdio".to_string()]),
                env: binary_env.unwrap_or_default(),
            });
        }

        let server_path = self.server_script_path(language_server_id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let lsp_settings =
            match zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
                Ok(settings) => settings,
                Err(_) => {
                    return Ok(Some(zed::serde_json::json!({"config": {}})));
                }
            };

        let mut initialization_options = lsp_settings
            .initialization_options
            .clone()
            .unwrap_or_else(|| zed::serde_json::json!({}));

        if !initialization_options.is_object() {
            initialization_options = zed::serde_json::json!({});
        }

        let markdownlint_settings = lsp_settings
            .settings
            .clone()
            .unwrap_or_else(|| zed::serde_json::json!({}));

        if let Some(options_obj) = initialization_options.as_object_mut() {
            options_obj.entry("config").or_insert(markdownlint_settings);
        }

        Ok(Some(initialization_options))
    }
}

zed::register_extension!(MarkdownlintExtension);
