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

    pub mod ssa {
        use std::collections::HashMap;

        use super::RightHandOperand;

        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        pub enum Name {
            X,
            Y,
            Z,
            W,
            Input,
        }
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        pub struct Val {
            name: Name,
            index: usize,
        }
        impl Val {
            pub fn as_node(&self, bindings: &HashMap<Val, i64>) -> String {
                let id = format!("{:?}{}", self.name, self.index);
                if let Some(b) = bindings.get(self) {
                    format!("{} [label={}]\n{}", id, b, id)
                } else {
                    id
                }
            }
        }
        #[derive(Debug)]
        pub enum RightHand {
            Val(Val),
            Number(i64, usize),
        }
        impl RightHand {
            pub fn as_node(&self, bindings: &HashMap<Val, i64>) -> String {
                match self {
                    RightHand::Val(v) => v.as_node(bindings),
                    RightHand::Number(n, i) => format!("{} [label={}]\n{}", i, n, i),
                }
            }
            pub fn is_val(&self) -> bool {
                match self {
                    RightHand::Val(_) => true,
                    _ => false,
                }
            }

            fn is_zero(&self) -> bool {
                match self {
                    RightHand::Val(_) => false,
                    RightHand::Number(n, _) => *n == 0,
                }
            }
        }
        #[derive(Debug)]
        pub enum Instruction {
            Input {
                idx: usize,
                from: Val,
                to: Val,
            }, // from here is next_input
            Add {
                idx: usize,
                from: Val,
                to: Val,
                rhs: RightHand,
            },
            Mul {
                idx: usize,
                from: Val,
                to: Val,
                rhs: RightHand,
            },
            Div {
                idx: usize,
                from: Val,
                to: Val,
                rhs: RightHand,
            },
            Mod {
                idx: usize,
                from: Val,
                to: Val,
                rhs: RightHand,
            },
            Eql {
                idx: usize,
                from: Val,
                to: Val,
                rhs: RightHand,
            },
        }
        impl Instruction {
            fn to(&self) -> &Val {
                match self {
                    Instruction::Input { to, .. } => &to,
                    Instruction::Add { to, .. } => &to,
                    Instruction::Mul { to, .. } => &to,
                    Instruction::Div { to, .. } => &to,
                    Instruction::Mod { to, .. } => &to,
                    Instruction::Eql { to, .. } => &to,
                }
            }
            pub fn as_dot(&self, bindings: &HashMap<Val, i64>) -> String {
                match self {
                    Instruction::Input { idx, from, to } => {
                        let in_node = format!("inp_{}", idx);
                        format!(
                            "{}\n{} -> {}\n{} -> {}",
                            in_node,
                            from.as_node(bindings),
                            in_node,
                            in_node,
                            to.as_node(bindings)
                        )
                    }
                    Instruction::Add { idx, from, to, rhs } => {
                        let in_node = format!("add_{}", idx);
                        format!(
                            "{}\n{} -> {}\n{} -> {}\n{} -> {}",
                            in_node,
                            from.as_node(bindings),
                            in_node,
                            in_node,
                            to.as_node(bindings),
                            rhs.as_node(bindings),
                            in_node
                        )
                    }
                    Instruction::Mul { idx, from, to, rhs } => {
                        let in_node = format!("mul_{}", idx);
                        format!(
                            "{} \n{} -> {}\n{} -> {}\n{} -> {}",
                            in_node,
                            from.as_node(bindings),
                            in_node,
                            in_node,
                            to.as_node(bindings),
                            rhs.as_node(bindings),
                            in_node
                        )
                    }
                    Instruction::Div { idx, from, to, rhs } => {
                        let in_node = format!("div_{}", idx);
                        format!(
                            "{} \n{} -> {}\n{} -> {}\n{} -> {}",
                            in_node,
                            from.as_node(bindings),
                            in_node,
                            in_node,
                            to.as_node(bindings),
                            rhs.as_node(bindings),
                            in_node
                        )
                    }
                    Instruction::Mod { idx, from, to, rhs } => {
                        let in_node = format!("mod_{}", idx);
                        format!(
                            "{} \n{} -> {}\n{} -> {}\n{} -> {}",
                            in_node,
                            from.as_node(bindings),
                            in_node,
                            in_node,
                            to.as_node(bindings),
                            rhs.as_node(bindings),
                            in_node
                        )
                    }
                    Instruction::Eql { idx, from, to, rhs } => {
                        let in_node = format!("eql_{}", idx);
                        format!(
                            "{} \n{} -> {}\n{} -> {}\n{} -> {}",
                            in_node,
                            from.as_node(bindings),
                            in_node,
                            in_node,
                            to.as_node(bindings),
                            rhs.as_node(bindings),
                            in_node
                        )
                    }
                }
            }

            pub fn map_rhs<F>(&self, mut f: F) -> Self
            where
                F: FnMut(&RightHand) -> RightHand,
            {
                match self {
                    Instruction::Input { idx, from, to } => Instruction::Input {
                        idx: *idx,
                        from: *from,
                        to: *to,
                    },
                    Instruction::Add { idx, from, to, rhs } => Instruction::Add {
                        idx: *idx,
                        from: *from,
                        to: *to,
                        rhs: f(rhs),
                    },
                    Instruction::Mul { idx, from, to, rhs } => Instruction::Mul {
                        idx: *idx,
                        from: *from,
                        to: *to,
                        rhs: f(rhs),
                    },
                    Instruction::Div { idx, from, to, rhs } => Instruction::Div {
                        idx: *idx,
                        from: *from,
                        to: *to,
                        rhs: f(rhs),
                    },
                    Instruction::Mod { idx, from, to, rhs } => Instruction::Mod {
                        idx: *idx,
                        from: *from,
                        to: *to,
                        rhs: f(rhs),
                    },
                    Instruction::Eql { idx, from, to, rhs } => Instruction::Eql {
                        idx: *idx,
                        from: *from,
                        to: *to,
                        rhs: f(rhs),
                    },
                }
            }
        }

        pub struct IndexState {
            x: usize,
            y: usize,
            z: usize,
            w: usize,
            input: usize,
            instr: usize,
            number: usize,
        }
        impl IndexState {
            fn new() -> IndexState {
                IndexState {
                    x: 0,
                    y: 0,
                    z: 0,
                    w: 0,
                    input: 0,
                    instr: 0,
                    number: 0,
                }
            }

            fn next_instr(&mut self) -> usize {
                self.instr += 1;
                self.instr
            }
            fn next_val(&mut self, var: &super::Var) -> Val {
                match var {
                    super::Var::X => {
                        self.x += 1;
                        Val {
                            name: Name::X,
                            index: self.x,
                        }
                    }
                    super::Var::Y => {
                        self.y += 1;
                        Val {
                            name: Name::Y,
                            index: self.y,
                        }
                    }
                    super::Var::Z => {
                        self.z += 1;
                        Val {
                            name: Name::Z,
                            index: self.z,
                        }
                    }
                    super::Var::W => {
                        self.w += 1;
                        Val {
                            name: Name::W,
                            index: self.w,
                        }
                    }
                }
            }

            fn current_val(&self, var: &super::Var) -> Val {
                match var {
                    super::Var::X => Val {
                        name: Name::X,
                        index: self.x,
                    },
                    super::Var::Y => Val {
                        name: Name::Y,
                        index: self.y,
                    },
                    super::Var::Z => Val {
                        name: Name::Z,
                        index: self.z,
                    },
                    super::Var::W => Val {
                        name: Name::W,
                        index: self.w,
                    },
                }
            }

            fn next_rhs(&mut self, rhs: &super::RightHandOperand) -> RightHand {
                match rhs {
                    RightHandOperand::Var(super::Var::X) => RightHand::Val(Val {
                        name: Name::X,
                        index: self.x,
                    }),
                    RightHandOperand::Var(super::Var::Y) => RightHand::Val(Val {
                        name: Name::Y,
                        index: self.y,
                    }),
                    RightHandOperand::Var(super::Var::Z) => RightHand::Val(Val {
                        name: Name::Z,
                        index: self.z,
                    }),
                    RightHandOperand::Var(super::Var::W) => RightHand::Val(Val {
                        name: Name::W,
                        index: self.w,
                    }),
                    RightHandOperand::Number(n) => {
                        let r = RightHand::Number(*n, self.number);
                        self.number += 1;
                        r
                    }
                }
            }

            fn next_number(&mut self) -> usize {
                self.number += 1;
                self.number
            }

            fn next_input(&mut self) -> Val {
                let r = Val {
                    name: Name::Input,
                    index: self.input,
                };
                self.input += 1;
                r
            }
        }

        pub fn from_straightline(instrs: Vec<super::Instruction>) -> (Vec<Instruction>, IndexState) {
            use Instruction::*;
            let mut state = IndexState::new();
            let ssa = instrs
                .iter()
                .map(|instr| match instr {
                    super::Instruction::Input(v) => Input {
                        idx: state.next_instr(),
                        to: state.next_val(v),
                        from: state.next_input(),
                    },
                    super::Instruction::Add(l, r) => Add {
                        idx: state.next_instr(),
                        from: state.current_val(l),
                        to: state.next_val(l),
                        rhs: state.next_rhs(r),
                    },
                    super::Instruction::Mul(l, r) => Mul {
                        idx: state.next_instr(),
                        from: state.current_val(l),
                        to: state.next_val(l),
                        rhs: state.next_rhs(r),
                    },
                    super::Instruction::Div(l, r) => Div {
                        idx: state.next_instr(),
                        from: state.current_val(l),
                        to: state.next_val(l),
                        rhs: state.next_rhs(r),
                    },
                    super::Instruction::Mod(l, r) => Mod {
                        idx: state.next_instr(),
                        from: state.current_val(l),
                        to: state.next_val(l),
                        rhs: state.next_rhs(r),
                    },
                    super::Instruction::Eql(l, r) => Eql {
                        idx: state.next_instr(),
                        from: state.current_val(l),
                        to: state.next_val(l),
                        rhs: state.next_rhs(r),
                    },
                })
                .collect();

            (ssa, state)
        }

        fn lookup_rhs(bindings: &HashMap<Val, i64>, rhs: &RightHand) -> Option<i64> {
            // there was shenanigans here about needing a lifetime specifier for Option<i64>
            // I'll just copy the i64 for now!
            match rhs {
                RightHand::Number(n, _) => Some(*n),
                RightHand::Val(v) => bindings.get(v).copied(),
            }
        }

        fn evaluate_constants(instrs: &[Instruction]) -> HashMap<Val, i64> {
            let mut bindings = HashMap::new();

            bindings.insert(Val { name: Name::X, index: 0}, 0);
            bindings.insert(Val { name: Name::Y, index: 0}, 0);
            bindings.insert(Val { name: Name::Z, index: 0}, 0);
            bindings.insert(Val { name: Name::W, index: 0}, 0);

            for instr in instrs {
                match instr {
                    Instruction::Add { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = lookup_rhs(&bindings, rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.insert(*to, f + r);
                        }
                    }
                    Instruction::Mul { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = lookup_rhs(&bindings, rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.insert(*to, f*r);
                        }
                    }
                    Instruction::Div { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = lookup_rhs(&bindings, rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.insert(*to, f/r);
                        }
                    }
                    Instruction::Mod { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = lookup_rhs(&bindings, rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.insert(*to, f % r);
                        }
                    }
                    Instruction::Eql { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = lookup_rhs(&bindings, rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            let b = if *f == r { 1 } else { 0 };
                            bindings.insert(*to, b);
                        }
                    }
                    _ => (),
                }
            }

            bindings
        }

        pub fn eliminate_constants(
            instrs: Vec<Instruction>,
            mut index_state: IndexState,
        ) -> (Vec<Instruction>, HashMap<Val, i64>) {
            let constant_bindings = evaluate_constants(&instrs);

            let elim = instrs
                .iter()
                .filter(|i| !constant_bindings.contains_key(i.to()))
                .map(|instr| {
                    instr.map_rhs(|r| match r {
                        RightHand::Number(n, i) => RightHand::Number(*n, *i),
                        RightHand::Val(v) => constant_bindings.get(&v).map_or_else(
                            || RightHand::Val(*v),
                            |n| RightHand::Number(*n, index_state.next_number()),
                        ),
                    })
                })
                .collect();

            (elim, constant_bindings)
        }

        #[cfg(test)]
        mod tests {
            use crate::day24::puzzle::parser;

            use super::*;

            fn aux_eliminate(input: &str) -> Vec<Instruction> {
                let straightline = parser::parse_input(input).unwrap();
                let (ssa, index_state) = from_straightline(straightline);
                eliminate_constants(ssa, index_state).0
            }
            #[test]
            fn test_constant_elimination() {
                assert_eq!(aux_eliminate("mul x 0").len(), 0);
                assert_eq!(aux_eliminate("add x 0").len(), 0);
                assert_eq!(aux_eliminate("div x 1").len(), 0);
                assert_eq!(aux_eliminate("mod x 1").len(), 0);
                assert_eq!(aux_eliminate("eql x 1").len(), 0);
                assert_eq!(aux_eliminate("mul x 0\nadd x 0").len(), 0);

                println!("{:?}",aux_eliminate("mul x 0\nadd x z\nmod x 26\ndiv z 1\n"));
                assert_eq!(aux_eliminate("mul x 0\nadd x z\nmod x 26\ndiv z 1\n").len(), 0);
            }

        }
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
            tuple((ws(var), ws(rhs)))(input)
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

        let (_, instrs) = parser::instructions(example).unwrap();
        assert_eq!(instrs.len(), 18)
    }
}
