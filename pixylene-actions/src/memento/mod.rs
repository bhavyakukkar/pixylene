mod action;
pub use action::Action;

mod action_manager;
pub use action_manager::ActionManager;

pub type ActionResult = Result<(), crate::ActionError>;
