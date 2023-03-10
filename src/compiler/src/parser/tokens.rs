
#[derive(Clone)]
pub enum TokenSymbol {
    EOF,
    Newline(u32, u32),
    Indent,
    Dedent,
}