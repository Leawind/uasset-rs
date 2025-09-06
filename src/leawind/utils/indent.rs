use std::fmt::{Debug, Display, Formatter};

pub struct Indented<'a, T: ?Sized> {
    indent: &'a str,
    value: &'a T,
}
impl<T> Indented<'_, T> {
    fn fmt_each_line(&self, f: &mut Formatter<'_>, lines: &str) -> core::fmt::Result {
        let mut lines = lines.lines();
        if let Some(first) = lines.next() {
            write!(f, "{}{}", self.indent, first)?;
            for line in lines {
                write!(f, "\n{}{}", self.indent, line)?;
            }
        }
        Ok(())
    }
}
impl<T: Display> Display for Indented<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let output = self.value.to_string();
        self.fmt_each_line(f, &output)?;
        Ok(())
    }
}
impl<T: Debug> Debug for Indented<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let output = format!("{:?}", self.value);
        self.fmt_each_line(f, &output)?;
        Ok(())
    }
}

pub trait Indent {
    fn indent<'a>(&'a self, indent: &'a str) -> Indented<'a, Self>;
}
impl<T> Indent for T {
    fn indent<'a>(&'a self, indent: &'a str) -> Indented<'a, Self> {
        Indented {
            indent,
            value: self,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indent() {
        let s = "hello world\nyou are awesome";
        println!("{}", s.indent("  "));
        println!("{:?}", s.indent("  "));
    }
}
