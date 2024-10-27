use std::{collections::HashMap, vec};

advent_of_code::solution!(2023, 19);

struct WorkflowStep {
    category: char,
    order: i8,
    threshold: u64,
    dest_workflow: String,
}

struct Workflow {
    steps: Vec<WorkflowStep>,
    end: String,
}

type Interval = (u64, u64);

trait Split
where
    Self: Sized,
{
    fn split(&self, value: u64) -> Option<(Self, Self)>;
    fn rev_split(&self, value: u64) -> Option<(Self, Self)>;
}

impl Split for Interval {
    fn split(&self, value: u64) -> Option<(Interval, Interval)> {
        if value < self.0 || value > self.1 {
            return None;
        }
        Some(((self.0, value), (value + 1, self.1)))
    }

    fn rev_split(&self, value: u64) -> Option<(Interval, Interval)> {
        if value < self.0 || value > self.1 {
            return None;
        }
        Some(((self.0, value - 1), (value, self.1)))
    }
}

struct State {
    workflow: String,
    step: usize,
    interval: [Interval; 4],
}

const CATEGORIES: [char; 4] = ['x', 'm', 'a', 's'];

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<HashMap<char, u64>>) {
    let (workflows_input, parts_input) = input.trim_end_matches('\n').split_once("\n\n").unwrap();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for w in workflows_input.split('\n') {
        let (name, rest) = w.split_once('{').unwrap();
        let steps_input = rest.trim_end_matches('}').split(',').collect::<Vec<_>>();
        let (end, steps_input) = steps_input.split_last().unwrap();

        let mut steps: Vec<WorkflowStep> = Vec::new();
        for s in steps_input {
            let category = s.chars().next().unwrap();
            let order = match s.chars().nth(1) {
                Some('<') => -1,
                Some('>') => 1,
                c => panic!("Unknown order char: {:#?}", c),
            };
            let (threshold, dest) = s[2..].split_once(':').unwrap();

            steps.push(WorkflowStep {
                category,
                order,
                threshold: threshold.parse().unwrap(),
                dest_workflow: dest.to_string(),
            });
        }

        workflows.insert(
            name.to_string(),
            Workflow {
                steps,
                end: end.to_string(),
            },
        );
    }

    let mut parts: Vec<HashMap<char, u64>> = Vec::new();
    for p in parts_input.split('\n') {
        let mut dict = HashMap::new();
        for cat in p[1..p.len() - 1].split(',') {
            let (name, value) = cat.split_once('=').unwrap();
            dict.insert(name.chars().next().unwrap(), value.parse::<u64>().unwrap());
        }

        parts.push(dict);
    }

    (workflows, parts)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, parts) = parse(input);
    let mut accepted = 0;

    for part in parts {
        let mut workflow = workflows.get("in").unwrap();

        'part_loop: loop {
            for step in &workflow.steps {
                let value = part.get(&step.category).unwrap();
                let order = step.order as i64;

                if order * *value as i64 > order * step.threshold as i64 {
                    match step.dest_workflow.as_str() {
                        "A" => {
                            accepted += part.values().sum::<u64>();
                            break 'part_loop;
                        }
                        "R" => break 'part_loop,
                        _ => (),
                    }

                    workflow = workflows.get(&step.dest_workflow).unwrap();
                    continue 'part_loop;
                }
            }

            match workflow.end.as_str() {
                "A" => {
                    accepted += part.values().sum::<u64>();
                    break 'part_loop;
                }
                "R" => break 'part_loop,
                w => workflow = workflows.get(w).unwrap(),
            }
        }
    }

    Some(accepted)
}

pub fn part_two(input: &str) -> Option<u64> {
    let workflows = parse(input).0;
    let mut accepted: u64 = 0;

    let mut stack: Vec<State> = vec![State {
        workflow: "in".to_string(),
        step: 0,
        interval: [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
    }];
    while let Some(state) = stack.pop() {
        // Break condition
        match state.workflow.as_str() {
            "A" => {
                accepted += (state.interval[0].1 - state.interval[0].0 + 1)
                    * (state.interval[1].1 - state.interval[1].0 + 1)
                    * (state.interval[2].1 - state.interval[2].0 + 1)
                    * (state.interval[3].1 - state.interval[3].0 + 1);
                continue;
            }
            "R" => {
                continue;
            }
            _ => (),
        }

        // Retrieve workflow
        let workflow = workflows.get(&state.workflow).unwrap();

        if state.step == workflow.steps.len() {
            // End of workflow
            stack.push(State {
                workflow: workflow.end.clone(),
                step: 0,
                interval: state.interval,
            });
        } else {
            let step = &workflow.steps[state.step];
            let idx = CATEGORIES.iter().position(|&c| c == step.category).unwrap();

            match step.order {
                1 => {
                    if state.interval[idx].1 <= step.threshold {
                        // Entire interval moves to the next workflow step
                        stack.push(State {
                            workflow: state.workflow,
                            step: state.step + 1,
                            interval: state.interval,
                        })
                    } else if state.interval[idx].0 > step.threshold {
                        // Entire interval moves to the next workflow
                        stack.push(State {
                            workflow: step.dest_workflow.clone(),
                            step: 0,
                            interval: state.interval,
                        })
                    } else {
                        // Interval is split
                        let (left, right) = state.interval[idx].split(step.threshold).unwrap();

                        let mut left_itv = state.interval;
                        left_itv[idx] = left;
                        stack.push(State {
                            workflow: state.workflow,
                            step: state.step + 1,
                            interval: left_itv,
                        });

                        let mut right_itv = state.interval;
                        right_itv[idx] = right;
                        stack.push(State {
                            workflow: step.dest_workflow.clone(),
                            step: 0,
                            interval: right_itv,
                        });
                    }
                }
                -1 => {
                    if state.interval[idx].0 >= step.threshold {
                        // Entire interval moves to the next workflow step
                        stack.push(State {
                            workflow: state.workflow,
                            step: state.step + 1,
                            interval: state.interval,
                        })
                    } else if state.interval[idx].1 < step.threshold {
                        // Entire interval moves to the next workflow
                        stack.push(State {
                            workflow: step.dest_workflow.clone(),
                            step: 0,
                            interval: state.interval,
                        })
                    } else {
                        // Interval is split
                        let (left, right) = state.interval[idx].rev_split(step.threshold).unwrap();

                        let mut left_itv = state.interval;
                        left_itv[idx] = left;
                        stack.push(State {
                            workflow: step.dest_workflow.clone(),
                            step: 0,
                            interval: left_itv,
                        });

                        let mut right_itv = state.interval;
                        right_itv[idx] = right;
                        stack.push(State {
                            workflow: state.workflow,
                            step: state.step + 1,
                            interval: right_itv,
                        });
                    }
                }
                o => panic!("Unknown order: {}", o),
            }
        }
    }

    Some(accepted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(449531));
    }

    #[test]
    fn example_part_two() {
        let result = part_two(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(167409079868000));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(122756210763577));
    }
}
