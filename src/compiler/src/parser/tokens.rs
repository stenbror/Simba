
#[derive(Clone, Debug, PartialEq)]
pub enum TokenSymbol {
    EOF,
    Newline(u32, u32),
    Indent,
    Dedent,
    Unit(u32, u32), // '()' = Void
    And(u32, u32),
    Or(u32, u32),
    Not(u32, u32),
    Fun(u32, u32), //  Lambda    =>   fun a b -> a + b
    Query(u32, u32), // '?'
    Colon(u32, u32),
    ColonAssign(u32, u32), // ':='
}