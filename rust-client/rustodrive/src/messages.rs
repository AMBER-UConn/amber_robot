use crate::commands::ODriveCommand;

pub struct ODriveMessage {
    thread_id: usize,
    axis_id: usize,
    command: ODriveCommand,
    data: [u8; 8],
}

impl ODriveMessage {
    fn can_id(&self) -> u16 {
        return (self.axis_id as u16) << 5 | (self.command.clone() as u16);
    }
}

pub enum ODriveError {
    FailedToSend,
}

pub enum ODriveResponse {
    Ok([u8; 8]),
    Err(ODriveError),
}
