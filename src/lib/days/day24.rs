pub mod puzzle {
    #[derive(Debug, Clone)]
    pub enum Var {
        X,
        Y,
        Z,
        W,
    }

    pub enum RightHandOperand {
        Var(Var),
        Number(i64),
    }
    pub enum Instruction {
        Input(Var),
        Add(Var, RightHandOperand),
        Mul(Var, RightHandOperand),
        Div(Var, RightHandOperand),
        Mod(Var, RightHandOperand),
        Eql(Var, RightHandOperand),
    }
    pub mod parser {
        use crate::util::nom_helpers::ws;
        use nom::{
            branch::alt,
            bytes::complete::tag,
            combinator::{map, value},
            multi::many1,
            sequence::{preceded, tuple},
            IResult,
        };

        use super::*;
        use Instruction::*;

        pub fn var(input: &str) -> IResult<&str, Var> {
            alt((
                value(Var::X, tag("x")),
                value(Var::Y, tag("y")),
                value(Var::Z, tag("z")),
                value(Var::W, tag("w")),
            ))(input)
        }

        pub fn rhs(input: &str) -> IResult<&str, RightHandOperand> {
            alt((
                map(var, |v| RightHandOperand::Var(v)),
                map(nom::character::complete::i64, |n| {
                    RightHandOperand::Number(n)
                }),
            ))(input)
        }

        pub fn var_rhs(input: &str) -> IResult<&str, (Var, RightHandOperand)> {
            tuple((var, rhs))(input)
        }

        pub fn instruction(input: &str) -> IResult<&str, Instruction> {
            alt((
                preceded(tag("inp "), map(var, |v| Input(v))),
                preceded(tag("add "), map(var_rhs, |(v, r)| Add(v, r))),
                preceded(tag("mul "), map(var_rhs, |(v, r)| Mul(v, r))),
                preceded(tag("div "), map(var_rhs, |(v, r)| Div(v, r))),
                preceded(tag("mod "), map(var_rhs, |(v, r)| Mod(v, r))),
                preceded(tag("eql "), map(var_rhs, |(v, r)| Eql(v, r))),
            ))(input)
        }

        pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
            many1(ws(instruction))(input)
        }

        pub fn parse_input(input: &str) -> Option<Vec<Instruction>> {
            let (_, instrs) = instructions(input).ok()?;
            Some(instrs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::puzzle::*;

    #[test]
    fn test_parser() {
        let example = r#"
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
        "#;

        parser::instructions(example).unwrap();
    }
}
