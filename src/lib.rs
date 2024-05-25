use zed_extension_api as zed;

struct ZedNanoExtension {
    // ... state
}

impl zed::Extension for ZedNanoExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn language_server_command(
        &mut self,
        config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "echo".into(),
            args: Default::default(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(ZedNanoExtension);
