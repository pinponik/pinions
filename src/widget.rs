use crate::*;

pub enum Widget {
    #[cfg(feature = "button")]
    Button<const L: usize> { label: Str<L> },
}
