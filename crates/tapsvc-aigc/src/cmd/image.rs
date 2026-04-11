use crate::cli::ImageCommand;

pub async fn handle(command: ImageCommand) -> anyhow::Result<()> {
    match command {
        ImageCommand::Generate { .. } => {
            todo!("image generate not yet implemented")
        }
    }
}
