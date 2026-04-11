use crate::cli::VideoCommand;

pub async fn handle(command: VideoCommand) -> anyhow::Result<()> {
    match command {
        VideoCommand::Generate { .. } => {
            todo!("video generate not yet implemented")
        }
    }
}
