mod cat;
mod says;

pub use cat::*;
pub use says::*;

pub(self) enum Format {
    Jpeg,
    TaggedJpeg(String),
    Gif,
}
