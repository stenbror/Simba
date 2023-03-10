
#[derive(Clone, Debug, PartialEq)]
pub enum TokenSymbol {
    EOF,
    Newline(u32, u32),
    Indent,
    Dedent,
    Unit(u32, u32) // '()' = Void
}