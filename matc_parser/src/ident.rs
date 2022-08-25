use crate::comment::comment;
use crate::prelude::*;

pub(crate) fn ident() -> impl Parser<String> {
    text::ident()
        .padded()
        .padded_by(comment().padded().repeated())
        .boxed()
}
