use crate::canframe::{CANRequest, CANResponse};

#[derive(Clone, PartialEq, Debug)]
pub struct ErrorResponse {
    pub request: CANRequest,
    pub err: ODriveError
}

#[derive(Clone, Debug, PartialEq)]
pub enum ODriveError {
    FailedToSend
}

pub type ODriveResponse = Result<ResponseType, ErrorResponse>;

#[derive(Clone, Debug, PartialEq)]
pub enum ResponseType {
    Body {request: CANRequest, response: CANResponse},
    Bodyless{ req: CANRequest},
}

impl ResponseType {
    pub fn body(self) -> (CANRequest, CANResponse) {
        match self {
            ResponseType::Body{ request: req, response: resp} => (req, resp),
            ResponseType::Bodyless { req: _} => {
                panic!("Write requests do not return a response body")
            }
        }
    }

    pub fn request(self) -> CANRequest {
        match self {
            ResponseType::Body { request: req, response: _resp} => req,
            ResponseType::Bodyless { req} => req,
        }
    }
}


pub trait ManyResponses {
    fn unwrap_all(self) -> Vec<ResponseType>;
}
impl ManyResponses for Vec<ODriveResponse> {
    /// This method calls `.expect()` on all responses.
    /// This will panic if a single response is an error
    fn unwrap_all(self) -> Vec<ResponseType> {
        let mut frames = Vec::new();

        for response in self.into_iter() {
            match response {
                Ok(res) => frames.push(res),
                Err(err) => panic!("Error ({:?}) with request {:?}", err.err, err.request),
            }
        }
        frames
    }
}
