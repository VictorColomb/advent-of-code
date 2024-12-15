#[derive(Clone)]
pub(super) struct Equation {
    pub result: u64,
    pub operands: Vec<u64>,
}

pub(super) fn parse(input: &str) -> Vec<Equation> {
    let mut equations = Vec::new();

    for line in input.trim_end_matches('\n').lines() {
        let (result, operands) = line.split_once(": ").unwrap();

        let result = result.parse::<u64>().unwrap();
        let operands = operands
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        equations.push(Equation { result, operands })
    }

    equations
}
