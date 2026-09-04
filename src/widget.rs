use crate::*;

pub enum Widget {
    #[cfg(feature = "button")]
    Button { label: String },
}
