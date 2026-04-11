use crate::cli::AudioCommand;

pub async fn handle(command: AudioCommand) -> anyhow::Result<()> {
    match command {
        AudioCommand::Speech { .. } => {
            todo!("audio speech not yet implemented")
        }
    }
}
