#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub line: usize,
    pub char: usize,
}

pub struct Debugger {
    pub source: String,
}

impl Debugger {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn byte_pos_to_location(&self, byte_pos: usize) -> Location {
        let mut line = 1;
        let mut char = 1;

        for (i, c) in self.source.char_indices() {
            if i >= byte_pos {
                break;
            }

            if c == '\n' {
                line += 1;
                char = 1;
            } else {
                char += 1;
            }
        }

        Location { line, char }
    }

    pub fn display(&self, span: usize) -> String {
        let loc = self.byte_pos_to_location(span);
        format!("Line: {}, Char: {}", loc.line, loc.char)
    }
}
