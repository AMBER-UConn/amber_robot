use crate::commands::ODriveCommand;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ODriveMessage {
    pub thread_id: usize,
    pub axis_id: usize,
    pub command: ODriveCommand,
    pub data: [u8; 8],
}

impl ODriveMessage {
    fn can_id(&self) -> u16 {
        return (self.axis_id as u16) << 5 | (self.command.clone() as u16);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ODriveError {
    FailedToSend,
}


#[derive(Clone, PartialEq, Debug)]
pub enum ODriveResponse {
    Ok([u8; 8]),
    Err(ODriveError),
}
