use chumsky::prelude::{Recursive, Simple};

pub(crate) type Rec<'a, T> = Recursive<'a, char, T, Simple<char>>;
