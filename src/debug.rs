use crate::ast::{Location, SourceSpan, Span};

pub fn byte_pos_to_location(source: &str, byte_pos: usize) -> Location {
    let mut line = 1;
    let mut char= 1;
    
    for (i, c) in source.char_indices() {
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

pub fn span_to_location(source: &str, span: &Span) -> SourceSpan {
    SourceSpan {
        start: byte_pos_to_location(source, span.start),
        end: byte_pos_to_location(source, span.end),
    }
}
