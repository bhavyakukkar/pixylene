use crate::Action;

use std::rc::Rc;
use std::cell::RefCell;


pub enum Change {
    Start,
    End,
    StartEnd(Rc<RefCell<dyn Action>>),
    Untracked(Rc<RefCell<dyn Action>>),
}
impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Change::*;
        match self {
            Start => write!(f, "Change::Start"),
            End => write!(f, "Change::End"),
            StartEnd(_) => write!(f, "Change::StartEnd"),
            Untracked(_) => write!(f, "Change::Untracked"),
        }
    }
}
impl std::fmt::Debug for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Change::*;
        match self {
            Start => write!(f, "S"),
            End => write!(f, "E"),
            StartEnd(_) => write!(f, "SE"),
            Untracked(_) => write!(f, "U"),
        }
    }
}
impl Change {
    pub fn as_untracked(self) -> Result<Self, ChangeError> {
        use UntrackError::*;
        match self {
            Change::Start |
            Change::End => Err(ChangeError::UntrackError(NotDefined(self))),
            Change::StartEnd(action_rc) |
            Change::Untracked(action_rc) => Ok(Change::Untracked(action_rc)),
        }
    }
}


// Error Types

#[derive(Debug)]
pub enum UntrackError {
    NotDefined(Change),
}
impl std::fmt::Display for UntrackError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use UntrackError::*;
        match self {
            NotDefined(change) => write!(
                f,
                "cannot set given change {} as untracked, only completed changes that contain an \
                action, like Change::StartEnd or another Change::Untracked, can be changed to \
                Change::Untracked",
                change,
            ),
        }
    }
}

#[derive(Debug)]
pub enum ChangeError {
    UntrackError(UntrackError),
}
impl std::fmt::Display for ChangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ChangeError::*;
        match self {
            UntrackError(untrack_error) => write!(f, "{}", untrack_error),
        }
    }
}
