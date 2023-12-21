use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
};

use num::Integer;

advent_of_code::solution!(2023, 20);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::High => std::fmt::Display::fmt("high", f),
            Pulse::Low => std::fmt::Display::fmt("low", f),
        }
    }
}

struct Signal {
    pulse: Pulse,
    origin: String,
    destination: String,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(
            &format!("{} --{}--> {}", self.origin, self.pulse, self.destination),
            f,
        )
    }
}

trait Component {
    fn receive_pulse(&mut self, origin: &str, pulse: Pulse) -> Option<Pulse>;
    fn targets(&self) -> Vec<String>;
    fn reset(&mut self);
    fn as_any(&mut self) -> &mut dyn Any;
}

/// This component has two states: on and off. When it receives a low pulse, it
/// toggles its state and emits a high pulse. When it receives a high pulse, it
/// does nothing.
struct FlipFlop {
    on: bool,
    targets: Vec<String>,
}

impl FlipFlop {
    fn new(targets: Vec<String>) -> Self {
        Self { on: false, targets }
    }
}

impl Component for FlipFlop {
    fn receive_pulse(&mut self, _origin: &str, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.on = !self.on;
                match self.on {
                    true => Some(Pulse::High),
                    false => Some(Pulse::Low),
                }
            }
        }
    }

    fn targets(&self) -> Vec<String> {
        self.targets.clone()
    }

    fn reset(&mut self) {
        self.on = false;
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
struct Conjunction {
    input_signals: HashMap<String, Pulse>,
    targets: Vec<String>,
}

impl Conjunction {
    fn new(inputs: &Vec<&String>, targets: Vec<String>) -> Self {
        let mut input_signals: HashMap<String, Pulse> = HashMap::new();
        for input in inputs {
            input_signals.insert(input.to_string(), Pulse::Low);
        }

        Self {
            input_signals,
            targets,
        }
    }

    fn add_input(&mut self, input: &str) {
        self.input_signals.insert(input.to_string(), Pulse::Low);
    }
}

impl Component for Conjunction {
    fn receive_pulse(&mut self, origin: &str, pulse: Pulse) -> Option<Pulse> {
        self.input_signals.insert(origin.to_owned(), pulse);

        if self
            .input_signals
            .values()
            .all(|&pulse| pulse == Pulse::High)
        {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn targets(&self) -> Vec<String> {
        self.targets.clone()
    }

    fn reset(&mut self) {
        for (_, pulse) in self.input_signals.iter_mut() {
            *pulse = Pulse::Low;
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct Broadcast {
    targets: Vec<String>,
}

impl Broadcast {
    fn new(targets: Vec<String>) -> Self {
        Self { targets }
    }
}

impl Component for Broadcast {
    fn receive_pulse(&mut self, _origin: &str, _pulse: Pulse) -> Option<Pulse> {
        Some(Pulse::Low)
    }

    fn targets(&self) -> Vec<String> {
        self.targets.clone()
    }

    fn reset(&mut self) {}

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
struct StopCondition {
    target: String,
    pulse: Pulse,
}

fn parse(input: &str) -> HashMap<String, Box<dyn Component>> {
    let mut components: HashMap<String, Box<dyn Component>> = HashMap::new();

    for line in input.trim_end_matches('\n').split('\n') {
        let (ex_name, targets) = line.split_once(" -> ").unwrap();
        let name = if ex_name == "broadcaster" {
            ex_name.to_string()
        } else {
            ex_name[1..].to_string()
        };
        let component_type = ex_name.chars().next().unwrap();
        let targets: Vec<String> = targets
            .split(", ")
            .map(|target| target.to_string())
            .collect();

        // Record inputs for already existing conjunctions
        for target in targets.iter() {
            if let Some(component) = components.get_mut(target) {
                if let Some(conjunction) = component.as_any().downcast_mut::<Conjunction>() {
                    conjunction.add_input(&name);
                }
            }
        }

        // Create and record component
        if ex_name == "broadcaster" {
            components.insert(name.clone(), Box::new(Broadcast::new(targets)));
        } else if component_type == '%' {
            components.insert(name.clone(), Box::new(FlipFlop::new(targets)));
        } else if component_type == '&' {
            let inputs = components
                .iter()
                .filter(|(_, c)| c.targets().contains(&name))
                .map(|(k, _)| k)
                .collect();

            components.insert(name.clone(), Box::new(Conjunction::new(&inputs, targets)));
        } else {
            panic!("Unknown component type: {}", component_type);
        }
    }

    components
}

fn press_button(
    components: &mut HashMap<String, Box<dyn Component>>,
    stop: Option<&StopCondition>,
) -> Option<(u32, u32)> {
    let mut nb_low = 1;
    let mut nb_high = 0;

    let mut stack = VecDeque::new();
    stack.push_back(Signal {
        pulse: Pulse::Low,
        origin: "button".to_string(),
        destination: "broadcaster".to_string(),
    });

    while let Some(signal) = stack.pop_front() {
        // Break
        if let Some(stop) = stop {
            if signal.origin == stop.target && signal.pulse == stop.pulse {
                return None;
            }
        }

        // Retrieve destination component and send pulse
        let component: &mut Box<dyn Component> = match components.get_mut(&signal.destination) {
            Some(component) => component,
            None => continue,
        };
        let next_pulse = component.receive_pulse(&signal.origin, signal.pulse);

        // Forward pulse to targets
        if let Some(next_pulse) = next_pulse {
            for target in component.targets() {
                stack.push_back(Signal {
                    pulse: next_pulse,
                    origin: signal.destination.clone(),
                    destination: target.clone(),
                });
                match next_pulse {
                    Pulse::High => nb_high += 1,
                    Pulse::Low => nb_low += 1,
                }
            }
        }
    }

    Some((nb_low, nb_high))
}

fn reset_components(components: &mut HashMap<String, Box<dyn Component>>) {
    for (_, component) in components.iter_mut() {
        component.reset();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut components = parse(input);
    let mut nb_low = 0;
    let mut nb_high = 0;

    // We could cache the state and calculate the answer through
    // an integer division, but this is already fast enough... 😇
    for _ in 0..1000 {
        let (low, high) = press_button(&mut components, None).unwrap();
        nb_low += low;
        nb_high += high;
    }

    Some(nb_low * nb_high)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut components = parse(input);

    // Find rx component precursor
    let rx_precursor = components
        .iter()
        .find(|(_, c)| c.targets().contains(&"rx".to_string()))
        .unwrap()
        .0
        .to_string();
    let rx_precursor = components
        .get_mut(&rx_precursor)
        .unwrap()
        .as_any()
        .downcast_mut::<Conjunction>()
        .unwrap()
        .clone();

    // Find minimum number of cycles for each rx pre-precursor to send a high pulse
    let mut cycles: Vec<u64> = Vec::new();
    for pre in rx_precursor.input_signals.keys() {
        reset_components(&mut components);

        let mut i = 1;
        while press_button(
            &mut components,
            Some(&StopCondition {
                target: pre.to_string(),
                pulse: Pulse::High,
            }),
        )
        .is_some()
        {
            i += 1;
        }

        cycles.push(i);
    }

    // LCM
    Some(cycles.iter().fold(1, |acc, x| (acc * x) / x.gcd(&acc)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::commands::{read_example, read_input};

    #[test]
    fn example1_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 1));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn example2_part_one() {
        let result = part_one(&read_example(YEAR, DAY, 2));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn input_part_one() {
        let result = part_one(&read_input(YEAR, DAY));
        assert_eq!(result, Some(949764474));
    }

    #[test]
    fn input_part_two() {
        let result = part_two(&read_input(YEAR, DAY));
        assert_eq!(result, Some(243221023462303));
    }
}
