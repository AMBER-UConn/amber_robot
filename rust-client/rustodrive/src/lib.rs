pub mod canproxy;
pub mod commands;
pub mod messages;
pub mod odrivegroup;
pub mod threads;
pub(crate) mod cansocket;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
