pub mod puzzle {
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
            combinator::map,
            multi::many1,
            sequence::{preceded, tuple},
            IResult,
        };

        use super::*;
        use Instruction::*;

        pub fn var(input: &str) -> IResult<&str, Var> {
            todo!();
        }

        pub fn rhs(input: &str) -> IResult<&str, RightHandOperand> {
            todo!()
        }

        pub fn var_rhs(input: &str) -> IResult<&str, (Var, RightHandOperand)> {
            tuple((var, rhs))(input)
        }

        pub fn instruction(input: &str) -> IResult<&str, Instruction> {
            alt((
                preceded(tag("inp "), map(var, |v| Input(v))),
                preceded(tag("add "), map(var_rhs, |(v, r)| Add(v, r))),
            ))(input)
        }

        pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
            many1(ws(instruction))(input)
        }
    }
}
