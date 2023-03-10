
#[derive(Clone, Debug, PartialEq)]
pub enum TokenSymbol {
    EOF,
    Newline(u32, u32),
    Indent,
    Dedent,
    Unit(u32, u32), // '()' = Void
    And(u32, u32),
    Or(u32, u32)
}