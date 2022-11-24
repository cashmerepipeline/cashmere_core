mod conversation;
mod conversations_map;
mod terminal_delegate;
mod terminal_delegates_map;

mod types;

pub use conversation::Conversation;
pub use terminal_delegate::TerminalDelegate;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
