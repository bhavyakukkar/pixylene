mod console;
pub use console::Console;

#[derive(Debug)]
pub enum LogType {
    Info,
    Error,
    Warning,
    Success,
}

#[cfg(test)]
mod tests {
}
