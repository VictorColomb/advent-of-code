use crate::parse::Equation;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}
static OPERATORS_1: [Operator; 2] = [Operator::Add, Operator::Multiply];
static OPERATORS_2: [Operator; 3] = [Operator::Add, Operator::Multiply, Operator::Concat];

impl Operator {
    fn apply(self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concat => (left.to_string() + &right.to_string())
                .parse::<u64>()
                .unwrap(),
        }
    }
}

fn recurse(target: u64, current: u64, operands: &[u64], part2: bool) -> bool {
    if operands.is_empty() {
        return target == current;
    }

    let operators = if part2 {
        OPERATORS_2.iter()
    } else {
        OPERATORS_1.iter()
    };
    for operator in operators {
        if recurse(
            target,
            operator.apply(current, operands[0]),
            &operands[1..],
            part2,
        ) {
            return true;
        }
    }

    false
}

pub(super) fn solve_equation(equation: &Equation, part2: bool) -> bool {
    recurse(
        equation.result,
        equation.operands[0],
        &equation.operands[1..],
        part2,
    )
}
