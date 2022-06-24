/// A macro that conditionally runs code depending on the feature that is enabled.
/// This is useful when you want to "duck-type" one struct with another for testing
/// purposes
/// https://stackoverflow.com/a/72744251/10521417
macro_rules! cfg_match {
    ( other => {$($tt:tt)*} ) => ( $($tt)* );
    ( $cfg:meta => $expansion:tt $(, $($rest:tt)+)? ) => (
        #[cfg($cfg)]
        cfg_match! { other => $expansion }
        $($(
            #[cfg(not($cfg))]
            cfg_match! { other => $rest }
        )?)?
    );
} use cfg_match;
use socketcan::CANSocketOpenError;

cfg_match! {
    feature = "mock-socket" => {
        /// Mock implementation
        pub(crate) struct CANSocket {}

        impl CANSocket {
            pub fn open(ifname: &str) -> Result<Self, CANSocketOpenError> {
                Ok(CANSocket{})
            }
        }   
    },
    other => {
        pub(crate) use socketcan::CANSocket;
    },
}