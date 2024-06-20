use std::fmt;

pub enum Actions {
    GET,
}

pub struct ChangelogIncludes {
    pub version: bool,
    pub body: bool,
    pub date: bool
}

pub enum Format {
    JSON,
}

pub struct GetInputs {
    pub filename: String,
    pub n: usize,
    pub includes: ChangelogIncludes,
    pub format: Format
}

impl ChangelogIncludes {
    pub fn all_false(&self) -> bool {
        !self.version && !self.body && !self.date
    }
    pub fn make_all_true(&mut self) {
        self.version = true;
        self.body = true;
        self.date = true;
    }
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Actions::GET => write!(f, "get"),
        }
    }
}

impl fmt::Debug for ChangelogIncludes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChangelogIncludes")
            .field("body", &self.body)
            .field("version", &self.version)
            .field("date", &self.date)
            .finish()
    }
}

impl fmt::Debug for GetInputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GetInputs")
            .field("filename", &self.filename)
            .field("n", &self.n)
            .field("includes", &self.includes)
            .finish()
    }
}

