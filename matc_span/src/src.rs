use internment::Intern;
use std::fmt;
use std::path::Path;

#[derive(Clone, PartialEq, Copy, Hash, Eq)]
pub struct SrcId(Intern<Vec<String>>);

impl SrcId {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        SrcId(Intern::new(
            path.as_ref()
                .iter()
                .map(|c| c.to_string_lossy().into_owned())
                .collect(),
        ))
    }

    pub fn any() -> Self {
        Self(Intern::new(Vec::new()))
    }
}

impl fmt::Debug for SrcId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.len() == 0 {
            write!(f, "unknown")
        } else {
            write!(f, "{}", self.0.clone().join("/"))
        }
    }
}

impl fmt::Display for SrcId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
