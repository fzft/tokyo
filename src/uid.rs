use uuid::Uuid as LibUuid;

pub struct CustomUuid(LibUuid);

impl Default for CustomUuid {
    fn default() -> Self {
        CustomUuid(LibUuid::new_v4())
    }
}

use std::fmt;

impl PartialEq for CustomUuid {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for CustomUuid {}

impl PartialOrd for CustomUuid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for CustomUuid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::hash::Hash for CustomUuid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl Clone for CustomUuid {
    fn clone(&self) -> Self {
        CustomUuid(self.0)
    }
}

impl Copy for CustomUuid {}

impl fmt::Debug for CustomUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for CustomUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}