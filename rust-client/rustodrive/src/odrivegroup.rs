use std::collections::{BTreeMap};

use crate::{
    messages::{CANRequest},
    threads::ReadWriteCANThread, axis::{AxisID, Axis}, response::ODriveResponse,
};
pub struct ODriveGroup<'a> {
    can: ReadWriteCANThread,
    axes: BTreeMap<&'a AxisID, Axis<'a>>,
}

impl<'a> ODriveGroup<'a> {
    pub fn new(can: ReadWriteCANThread, axis_ids: &'static [AxisID]) -> Self {
        ODriveGroup {
            axes: axis_ids.iter().map(|id| (id, Axis::new(id))).collect(),
            can,
        }
    }

    pub fn all_axes<F>(&self, mut f: F) -> Vec<ODriveResponse>
    where
        F: FnMut(&Axis) -> CANRequest,
    {   
        let requests = self.axes.values().map(|ax| f(ax)).collect();
        self.can.request_many(requests)
    }

    pub fn axis<F>(&self, axis_id: &AxisID, f: F) -> ODriveResponse
    where
        F: FnOnce(&Axis) -> CANRequest,
    {
        self.can.request(f(self.get_axis(axis_id)))
    }
    
    fn get_axis(&self, id: &AxisID) -> &Axis {
        match self.axes.get(id) {
            Some(axis) => axis,
            None => panic!("Cannot retrieve axis {} that doesn't exist!", id)
        }
    }

}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::canproxy::CANProxy;
    use crate::messages::{CANRequest};
    use crate::response::ResponseType;
    use crate::tests::{wait_for_msgs};
    use crate::commands::{ODriveAxisState::*, ODriveCommand, Write};

    use super::ODriveGroup;

    #[test]
    fn test_all_axes() {
        let mut proxy = CANProxy::new("fakecan");

        let (send, rcv) = channel();
        
        let mut expected_requests = Vec::new();
        for i in 0..6 {
            expected_requests.push(CANRequest {axis: i, cmd: ODriveCommand::Write(Write::SetAxisRequestedState), data: [FullCalibrationSequence as u8, 0, 0, 0, 0, 0, 0, 0]})
        }
        
        proxy.register_rw("thread 1", move |can_rw| {
            let odrives = ODriveGroup::new(can_rw, &[0, 1, 2, 3, 4, 5]);

            let responses = odrives.all_axes(|ax| ax.set_state(FullCalibrationSequence));
            send.send(responses);
        });
        let stop_all = proxy.begin();

        // test the that all the results are returned in the order they were sent 
        let response = wait_for_msgs(rcv);
        stop_all().unwrap();

        
        for (request, response) in expected_requests.into_iter().zip(response) {
            assert_eq!(response, Ok(ResponseType::Bodyless{req: request}));
        }

    }
}