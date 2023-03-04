pub mod puzzle {
    #[derive(Debug, Clone, PartialEq, Eq)]
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

    pub struct ALU {
        x: i64,
        y: i64,
        pub z: i64,
        w: i64,
    }
    impl ALU {
        pub fn new() -> ALU {
            ALU {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            }
        }
        fn select_reg(&mut self, r: &Var) -> &mut i64 {
            if *r == Var::X {
                println!("Accessing ALU.z {}", self.z);
            }
            match r {
                Var::X => &mut self.x,
                Var::Y => &mut self.y,
                Var::Z => &mut self.z,
                Var::W => &mut self.w,
            }
        }

        fn binop<F>(&mut self, l: &Var, r: &RightHandOperand, f: F)
        where
            F: Fn(i64, i64) -> i64,
        {
            let orig = *self.select_reg(l);
            let rhs: i64 = match r {
                RightHandOperand::Var(v) => *self.select_reg(v),
                RightHandOperand::Number(n) => *n,
            };
            *self.select_reg(l) = f(orig, rhs);
        }
        pub fn run(&mut self, instrs: &[Instruction], mut digits: Vec<i64>) {
            digits.reverse();
            for instr in instrs {
                match instr {
                    Instruction::Input(l) => *self.select_reg(l) = digits.pop().unwrap(),
                    Instruction::Add(l, r) => self.binop(l, r, |a, b| a + b),
                    Instruction::Mul(l, r) => self.binop(l, r, |a, b| a * b),
                    Instruction::Div(l, r) => self.binop(l, r, |a, b| a / b),
                    Instruction::Mod(l, r) => self.binop(l, r, |a, b| a % b),
                    Instruction::Eql(l, r) => self.binop(l, r, |a, b| if a == b { 1 } else { 0 }),
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct SectionParam {
        a: i64,
        b: i64,
        c: i64,
    }

    // This is the main thrust of the puzzle
    // The input has 14 sections which vary only by the a, b, c parameters
    // The digit is provided as d
    // And z is the only state carried between sections
    fn emulate_section(a: i64, b: i64, c: i64, d: i64, z: i64) -> i64 {
        let nz = z / a;
        let cmp = if (z % 26 + b) == d { 0 } else { 1 };
        (26 * cmp + 1) * nz + (c + d) * cmp
    }

    pub fn emulate_sections(digits: &[i64], params: &[SectionParam]) -> i64 {
        let mut z = 0;

        for (i, SectionParam { a, b, c }) in params.iter().enumerate() {
            z = emulate_section(*a, *b, *c, digits[i], z);
        }

        z
    }

    #[cfg(test)]
    mod tests {
        use super::{emulate_section, parser::parse_input, ALU};

        fn aux_run(input: &str, digits: Vec<i64>) -> i64 {
            let instrs = parse_input(input).unwrap();
            let mut alu = ALU::new();
            alu.run(&instrs, digits);

            alu.z
        }

        #[test]
        fn test_alu() {
            let example = "
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
            ";

            assert_eq!(aux_run(example, vec![1]), 9);
            assert_eq!(aux_run(example, vec![1]), emulate_section(1, 13, 8, 1, 0));
        }
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
            pub name: Name,
            pub index: usize,
        }
        impl Val {
            pub fn as_node(&self, bindings: &Bindings) -> String {
                let id = format!("{:?}{}", self.name, self.index);
                if let Some(b) = bindings.get_exact(self) {
                    format!("{} [label={}]\n{}", id, b, id)
                } else {
                    if self.name == Name::Input {
                        format!("{} [color=red]\n{}", id, id)
                    } else {
                        id
                    }
                }
            }
        }
        #[derive(Debug, Clone)]
        pub enum RightHand {
            Val(Val),
            Number(i64, usize),
        }
        impl RightHand {
            pub fn as_node(&self, bindings: &Bindings) -> String {
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
            fn is_const(&self, c: i64) -> bool {
                match self {
                    RightHand::Val(_) => false,
                    RightHand::Number(n, _) => *n == c,
                }
            }

            fn is_zero(&self) -> bool {
                self.is_const(0)
            }
        }
        #[derive(Debug, Clone)]
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
            pub fn to(&self) -> &Val {
                match self {
                    Instruction::Input { to, .. } => &to,
                    Instruction::Add { to, .. } => &to,
                    Instruction::Mul { to, .. } => &to,
                    Instruction::Div { to, .. } => &to,
                    Instruction::Mod { to, .. } => &to,
                    Instruction::Eql { to, .. } => &to,
                }
            }
            pub fn as_dot(&self, bindings: &Bindings) -> String {
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

        pub fn from_straightline(instrs: &[super::Instruction]) -> (Vec<Instruction>, IndexState) {
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

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Constant {
            Exact(i64),
            Range { min: i64, max: i64 },
        }
        impl Constant {
            pub fn add(&self, o: &Constant) -> Self {
                match (self, o) {
                    (Constant::Exact(l), Constant::Exact(r)) => Constant::Exact(l + r),
                    (Constant::Exact(l), Constant::Range { min, max }) => Constant::Range {
                        min: min + l,
                        max: max + l,
                    },
                    (Constant::Range { min, max }, Constant::Exact(r)) => Constant::Range {
                        min: min + r,
                        max: max + r,
                    },
                    (
                        Constant::Range {
                            min: lmin,
                            max: lmax,
                        },
                        Constant::Range {
                            min: rmin,
                            max: rmax,
                        },
                    ) => Constant::Range {
                        min: lmin + rmin,
                        max: lmax + rmax,
                    },
                }
            }

            // mul
            pub fn mul(&self, o: &Constant) -> Self {
                match (self, o) {
                    (Constant::Exact(l), Constant::Exact(r)) => Constant::Exact(l * r),
                    (Constant::Exact(l), Constant::Range { min, max }) => Constant::Range {
                        min: min * l,
                        max: max * l,
                    },
                    (Constant::Range { min, max }, Constant::Exact(r)) => Constant::Range {
                        min: min * r,
                        max: max * r,
                    },
                    (
                        Constant::Range {
                            min: lmin,
                            max: lmax,
                        },
                        Constant::Range {
                            min: rmin,
                            max: rmax,
                        },
                    ) => Constant::Range {
                        min: lmin * rmin,
                        max: lmax * rmax,
                    },
                }
            }
            // div
            pub fn div(&self, o: &Constant) -> Self {
                match (self, o) {
                    (Constant::Exact(l), Constant::Exact(r)) => Constant::Exact(l / r),
                    (Constant::Exact(l), Constant::Range { min, max }) => Constant::Range {
                        min: min / l,
                        max: max / l,
                    },
                    (Constant::Range { min, max }, Constant::Exact(r)) => {
                        if max < r {
                            Constant::Exact(0)
                        } else {
                            Constant::Range {
                                min: min / r,
                                max: max / r,
                            }
                        }
                    }
                    (
                        Constant::Range {
                            min: lmin,
                            max: lmax,
                        },
                        Constant::Range {
                            min: rmin,
                            max: rmax,
                        },
                    ) => {
                        if lmax < rmax {
                            Constant::Exact(0)
                        } else {
                            Constant::Range {
                                min: lmin / rmax,
                                max: lmax / rmin,
                            }
                        }
                    }
                }
            }
            // mod
            pub fn modulo(&self, o: &Constant) -> Self {
                match (self, o) {
                    (Constant::Exact(l), Constant::Exact(r)) => Constant::Exact(l % r),
                    (Constant::Exact(l), Constant::Range { min, max }) => Constant::Range {
                        min: 0,
                        max: max - 1,
                    },
                    (Constant::Range { min, max }, Constant::Exact(r)) => {
                        Constant::Range { min: 0, max: r - 1 }
                    }
                    (
                        Constant::Range {
                            min: lmin,
                            max: lmax,
                        },
                        Constant::Range {
                            min: rmin,
                            max: rmax,
                        },
                    ) => Constant::Range {
                        min: 0,
                        max: rmax - 1,
                    },
                }
            }
            // eql
            pub fn eql(&self, o: &Constant) -> Self {
                match (self, o) {
                    (Constant::Exact(l), Constant::Exact(r)) => {
                        Constant::Exact(if l == r { 1 } else { 0 })
                    }
                    (Constant::Exact(l), Constant::Range { min, max }) => {
                        if l > max || l < min {
                            Constant::Exact(0)
                        } else {
                            Constant::Range { min: 0, max: 1 }
                        }
                    }
                    (Constant::Range { min, max }, Constant::Exact(r)) => {
                        if r > max || r < min {
                            Constant::Exact(0)
                        } else {
                            Constant::Range { min: 0, max: 1 }
                        }
                    }
                    (
                        Constant::Range {
                            min: lmin,
                            max: lmax,
                        },
                        Constant::Range {
                            min: rmin,
                            max: rmax,
                        },
                    ) => {
                        if (lmin <= rmin && lmax >= rmin)
                            || (lmin <= rmax && lmax >= rmax)
                            || (rmin <= lmin && rmax >= lmin)
                            || (rmin <= lmax && rmax >= lmax)
                        {
                            //overlapping
                            Constant::Range { min: 0, max: 1 }
                        } else {
                            Constant::Exact(0)
                        }
                    }
                }
            }
        }
        pub struct Bindings {
            b: HashMap<Val, Constant>,
        }
        impl Bindings {
            pub fn new() -> Bindings {
                let mut bindings = Bindings { b: HashMap::new() };

                bindings.add_exact(
                    Val {
                        name: Name::X,
                        index: 0,
                    },
                    0,
                );
                bindings.add_exact(
                    Val {
                        name: Name::Y,
                        index: 0,
                    },
                    0,
                );
                bindings.add_exact(
                    Val {
                        name: Name::Z,
                        index: 0,
                    },
                    0,
                );
                bindings.add_exact(
                    Val {
                        name: Name::W,
                        index: 0,
                    },
                    0,
                );

                bindings
            }
            pub fn add_exact(&mut self, v: Val, n: i64) {
                self.add(v, Constant::Exact(n));
            }

            pub fn add(&mut self, v: Val, constant: Constant) {
                if v.name == Name::X && v.index == 11 {
                    println!("add(X11, {:?})", constant);
                }
                self.b.insert(v, constant);
            }

            pub fn get_exact<'a>(&'a self, v: &Val) -> Option<&'a i64> {
                self.b.get(&v).and_then(|b| match b {
                    Constant::Exact(n) => Some(n),
                    Constant::Range { .. } => None,
                })
            }

            pub fn get(&self, v: &Val) -> Option<Constant> {
                self.b.get(&v).cloned()
            }

            pub fn get_rhs(&self, rhs: &RightHand) -> Option<Constant> {
                match rhs {
                    RightHand::Val(v) => self.get(v),
                    RightHand::Number(n, _) => Some(Constant::Exact(*n)),
                }
            }

            pub fn lookup_rhs_exact(&self, rhs: &RightHand) -> Option<i64> {
                match rhs {
                    RightHand::Val(v) => self.get_exact(v).copied(),
                    RightHand::Number(n, _) => Some(*n),
                }
            }

            fn has_exact(&self, v: &Val) -> bool {
                self.get_exact(v).is_some()
            }

            fn add_range(&mut self, v: Val, min: i64, max: i64) {
                self.b.insert(v, Constant::Range { min, max });
            }
        }

        fn evaluate_constants(instrs: &[Instruction]) -> Bindings {
            let mut bindings = Bindings::new();

            for instr in instrs {
                match instr {
                    Instruction::Add { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = bindings.get_rhs(rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.add(*to, f.add(&r));
                        }
                    }
                    Instruction::Mul { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = bindings.get_rhs(rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.add(*to, f.mul(&r));
                        }
                        if rhs.is_zero() {
                            bindings.add_exact(*to, 0);
                        }
                    }
                    Instruction::Div { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = bindings.get_rhs(rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.add(*to, f.div(&r));
                        }
                    }
                    Instruction::Mod { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = bindings.get_rhs(rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.add(*to, f.modulo(&r));
                        }
                    }
                    Instruction::Eql { from, to, rhs, .. } => {
                        let fromb = bindings.get(from);
                        let rhsb = bindings.get_rhs(rhs);
                        if let (Some(f), Some(r)) = (fromb, rhsb) {
                            bindings.add(*to, f.eql(&r));
                        }
                    }
                    Instruction::Input { to, .. } => bindings.add_range(*to, 1, 9),
                }
            }

            bindings
        }

        pub fn eliminate_constants(
            instrs: &[Instruction],
            mut index_state: IndexState,
        ) -> (Vec<Instruction>, Bindings) {
            let constant_bindings = evaluate_constants(&instrs);

            let elim = instrs
                .iter()
                .filter(|i| !constant_bindings.has_exact(i.to()))
                .map(|instr| {
                    instr.map_rhs(|r| match r {
                        RightHand::Number(n, i) => RightHand::Number(*n, *i),
                        RightHand::Val(v) => constant_bindings.get_exact(&v).map_or_else(
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
                aux_eval(input).0
            }

            fn aux_eval(input: &str) -> (Vec<Instruction>, Bindings) {
                let straightline = parser::parse_input(input).unwrap();
                let (ssa, index_state) = from_straightline(straightline);
                eliminate_constants(&ssa, index_state)
            }
            #[test]
            fn test_constant_elimination() {
                assert_eq!(aux_eliminate("mul x 0").len(), 0);
                assert_eq!(aux_eliminate("add x 0").len(), 0);
                assert_eq!(aux_eliminate("div x 1").len(), 0);
                assert_eq!(aux_eliminate("mod x 1").len(), 0);
                assert_eq!(aux_eliminate("eql x 1").len(), 0);
                assert_eq!(aux_eliminate("mul x 0\nadd x 0").len(), 0);
                assert_eq!(
                    aux_eliminate("mul x 0\nadd x z\nmod x 26\ndiv z 1\n").len(),
                    0
                );
                assert_eq!(aux_eliminate("inp w\nadd x w\nmul x 0").len(), 2);
                let (_, b) = aux_eval("inp w\nadd x w\neql x 13");
                for (k, v) in b.b.iter() {
                    println!("{:?} {:?}", k, v);
                }
                assert_eq!(aux_eliminate("inp w\nadd x w\neql x 13").len(), 2);
            }

            fn aux_is_false(l: &Constant, r: &Constant) {
                assert!(
                    l.eql(r) == Constant::Exact(0),
                    "{:?} == {:?} should be false",
                    l,
                    r
                );
            }

            fn aux_is_possibly_true(l: &Constant, r: &Constant) {
                assert!(
                    l.eql(r) == Constant::Range { min: 0, max: 1 },
                    "{:?} == {:?} should be possibly true",
                    l,
                    r
                );
            }
            #[test]
            fn test_constant_eql() {
                aux_is_false(
                    &Constant::Range { min: 0, max: 1 },
                    &Constant::Range { min: 3, max: 4 },
                );
                aux_is_possibly_true(
                    &Constant::Range { min: 0, max: 1 },
                    &Constant::Range { min: 1, max: 2 },
                );
            }
        }
    }

    pub mod tree {
        use std::collections::HashMap;

        use super::ssa::{self, Bindings, Constant, RightHand, Val};
        use ssa::Instruction;
        use std::fmt::Write;

        #[derive(Debug, Clone)]
        pub enum Tree {
            Input { index: usize },
            Constant { value: i64 },
            Eql { l: Box<Tree>, r: Box<Tree> },
            Add { l: Box<Tree>, r: Box<Tree> },
            Mul { l: Box<Tree>, r: Box<Tree> },
            Div { n: Box<Tree>, d: Box<Tree> },
            Mod { n: Box<Tree>, d: Box<Tree> },
        }

        struct TreeBuilder<'a> {
            instrs: &'a HashMap<Val, Instruction>,
            bindings: &'a Bindings,
        }
        impl TreeBuilder<'_> {
            fn for_val(&mut self, v: &Val) -> Box<Tree> {
                if let Some(exact) = self.bindings.get_exact(v) {
                    Box::new(Tree::Constant { value: *exact })
                } else {
                    Box::new(self.build(self.instrs.get(v).unwrap()))
                }
            }
            fn for_rhs(&mut self, rhs: &RightHand) -> Box<Tree> {
                match rhs {
                    RightHand::Val(v) => self.for_val(v),
                    RightHand::Number(n, _) => Box::new(Tree::Constant { value: *n }),
                }
            }
            fn build(&mut self, i: &Instruction) -> Tree {
                match i {
                    Instruction::Input { from, .. } => Tree::Input { index: from.index },
                    Instruction::Add { from, rhs, .. } => Tree::Add {
                        l: self.for_val(from),
                        r: self.for_rhs(rhs),
                    },
                    Instruction::Mul { from, rhs, .. } => Tree::Mul {
                        l: self.for_val(from),
                        r: self.for_rhs(rhs),
                    },
                    Instruction::Div { from, rhs, .. } => Tree::Div {
                        n: self.for_val(from),
                        d: self.for_rhs(rhs),
                    },
                    Instruction::Mod { from, rhs, .. } => Tree::Mod {
                        n: self.for_val(from),
                        d: self.for_rhs(rhs),
                    },
                    Instruction::Eql { from, rhs, .. } => Tree::Eql {
                        l: self.for_val(from),
                        r: self.for_rhs(rhs),
                    },
                }
            }
        }

        struct TreePrinter {
            i: usize,
            acc: String,
        }
        impl TreePrinter {
            fn id(&mut self, prefix: &str) -> String {
                let id = format!("{}-{}", prefix, self.i);
                self.i += 1;
                id
            }

            fn binop(&mut self, prefix: &str, l: &Tree, r: &Tree) -> String {
                let id = self.id(prefix);
                let lid = self.print(l);
                let rid = self.print(r);

                write!(&mut self.acc, "{} -> {}\n{} -> {}\n", id, lid, id, rid).unwrap();
                id
            }
            fn print(&mut self, t: &Tree) -> String {
                match t {
                    Tree::Input { index } => format!("input-{}", index),
                    Tree::Constant { value } => {
                        let id = self.id("constant");
                        write!(&mut self.acc, "{} [label={}]\n", id, value).unwrap();
                        id
                    }
                    Tree::Eql { l, r } => self.binop("eql", l, r),
                    Tree::Add { l, r } => self.binop("add", l, r),
                    Tree::Mul { l, r } => self.binop("mul", l, r),
                    Tree::Div { n, d } => self.binop("div", n, d),
                    Tree::Mod { n, d } => self.binop("mod", n, d),
                }
            }

            fn new() -> TreePrinter {
                TreePrinter {
                    i: 0,
                    acc: "".to_string(),
                }
            }
        }

        pub fn print_tree(t: &Tree) -> String {
            let mut printer = TreePrinter::new();

            printer.print(t);
            printer.acc
        }

        pub fn from_eliminated_ssa(ssa: &[Instruction], bindings: &Bindings) -> Tree {
            let mut binding_instr: HashMap<Val, Instruction> = HashMap::new();
            for instr in ssa {
                binding_instr.insert(instr.to().clone(), instr.clone());
            }

            let max_z = binding_instr
                .keys()
                .filter(|v| v.name == ssa::Name::Z)
                .max_by_key(|v| v.index)
                .unwrap();

            let mut builder = TreeBuilder {
                instrs: &binding_instr,
                bindings: bindings,
            };

            builder.build(binding_instr.get(max_z).unwrap())
        }
    }
    pub mod parser {
        use crate::util::nom_helpers::ws;
        use itertools::Itertools;
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

        fn get_constant(input: &str) -> Option<i64> {
            println!("get_constant({})", input);
            input.split(" ").last().map(|i| i.parse().ok()).flatten()
        }

        pub fn extract_params(input: &str) -> Vec<SectionParam> {
            input
                .lines()
                .enumerate()
                .filter_map(|(i, l)| {
                    if i % 18 == 4 || i % 18 == 5 || i % 18 == 15 {
                        Some(l)
                    } else {
                        None
                    }
                })
                .tuples()
                .map(|(a, b, c)| {
                    Some(SectionParam {
                        a: get_constant(a)?,
                        b: get_constant(b)?,
                        c: get_constant(c)?,
                    })
                })
                .flatten()
                .collect()
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
