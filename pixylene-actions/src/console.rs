/// A portable type to enable basic I/O from an Action to a defined Pixylene User Interface.
/// 
/// A shared Console instance is passed to an Action's perform_action method to enable the
/// Action to interact with a Pixylene user
/*
#[derive(Clone)]
pub struct Console<'a> {

    /// Closure called by an Action when it requires user input; return [`None`] if user refuses
    pub cmdin: Box<dyn Fn(String) -> Option<String> + 'a>,

    /// Closure called by an Action when it desires user notification
    pub cmdout: Box<dyn Fn(String, LogType) -> () + 'a>,
}
*/

pub trait Console {
    fn cmdin(&self, message: &str) -> Option<String>;
    fn cmdout(&self, message: &str, log_type: &LogType);
}

/// Nature of the message outputted by an Action
#[derive(Clone, Copy, Debug)]
pub enum LogType {
    Info,
    Error,
    Warning,
    Success,
}
