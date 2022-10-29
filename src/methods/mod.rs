mod cat;
mod says;

pub use cat::*;
pub use says::*;

#[derive(Debug)]
pub(self) enum Format {
    Jpeg,
    TaggedJpeg(String),
    Gif,
}
