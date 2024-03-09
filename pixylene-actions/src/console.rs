/// A portable type to enable basic I/O from an [`Action`][a] to a defined Pixylene User Interface.
/// 
/// A shared Console instance is passed to an Action's [`perform_action`][p] method to enable the
/// Action to interact with a Pixylene user
///
/// [a]: crate::Action
/// [p]: crate::Action::perform_action
pub struct Console<'a> {

    /// Closure called by an Action when it requires user input; return [`None`] if user refuses
    pub cmdin: Box<dyn Fn(String) -> Option<String> + 'a>,

    /// Closure called by an Action when it desires user notification
    pub cmdout: Box<dyn Fn(String, LogType) -> () + 'a>,
}

/*
pub trait Console {
    fn cmdin(&self, message: &str) -> Option<String>;
    fn cmdout(&self, message: &str, log_type: LogType);
}
*/

/// Nature of the message outputted by an Action
#[derive(Debug)]
pub enum LogType {
    Info,
    Error,
    Warning,
    Success,
}
