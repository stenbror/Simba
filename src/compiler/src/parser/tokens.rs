
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
    Including(u32, u32),
    Excluding(u32, u32),
    Is(u32, u32),
    Less(u32, u32),
    LessEqual(u32, u32),
    Equal(u32, u32),
    GreaterEqual(u32, u32),
    Greater(u32, u32),
    NotEqual(u32, u32),
    BitwiseOr(u32, u32),
    BitwiseXor(u32, u32),
    BitwiseAnd(u32, u32),
}