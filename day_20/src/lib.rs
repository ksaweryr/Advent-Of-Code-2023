#![feature(iter_intersperse)]

use std::{collections::{HashMap, VecDeque}, ops::Neg, str::FromStr};

use anyhow::Error;

pub fn solve(input: String) {
    let mut circuit = parse_input(&input);

    println!("{}", part1(&mut circuit));
    println!("{}", part2(&circuit));
}

fn parse_input(input: &str) -> Circuit {
    input.parse().expect("Invalid input")
}

fn part1(circuit: &mut Circuit) -> usize {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let (low_delta, high_delta) = circuit.push_button();
        low_pulses += low_delta;
        high_pulses += high_delta;
    }

    low_pulses * high_pulses
}

fn part2(circuit: &Circuit) -> usize {
    // Draw a graph of the input. The whole circuit is just 4 12-bit binary counters connected to a conjunction at the end, which only sends
    // a 0 to rx if all of the counters reach 0 at the same time. Find the period of each counter, the answer is LCM of those periods.
    circuit.outputs["broadcaster"].iter().map(|cs| max_counter_value(circuit, &cs, 1, 1))
        .reduce(lcm)
        .unwrap()
}

fn max_counter_value(circuit: &Circuit, node: &str, acc: usize, i: usize) -> usize {
    let next_node = circuit.outputs[node].iter()
        .find(|name| if let ModuleType::Conjunction(_) = circuit.modules[*name].module_type { false } else { true } );
    
    
    if let Some(next_node) = next_node {
        let bit = (circuit.outputs[next_node].len() + 1) % 2;
        max_counter_value(circuit, next_node, acc + (bit << i), i + 1)
    } else {
        acc + (1 << (i - 1))
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let (a, b) = if a < b {
        (b, a)
    } else {
        (a, b)
    };

    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pulse {
    Low,
    High
}

impl Neg for Pulse {
    type Output = Pulse;

    fn neg(self) -> Self::Output {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low
        }
    }
}

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(Pulse),
    Conjunction(Vec<Pulse>),
    Output
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    module_type: ModuleType
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "broadcaster" {
            return Ok(Module { name: s.to_owned(), module_type: ModuleType::Broadcaster })
        }

        match s.chars().nth(0).ok_or(Error::msg("Can't parse empty string"))? {
            '%' => Ok(Module { name: s[1..].to_owned(), module_type: ModuleType::FlipFlop(Pulse::Low) }),
            '&' => Ok(Module { name: s[1..].to_owned(), module_type: ModuleType::Conjunction(Vec::new()) }),
            c => Err(Error::msg(format!("Invalid module type: `{}`", c)))
        }
    }
}

impl Module {
    fn handle_pulse(&mut self, sourceno: usize, pulse: Pulse) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::Broadcaster => Some(pulse),
            ModuleType::FlipFlop(state) => if pulse == Pulse::Low {
                *state = -*state;
                Some(*state)
            } else {
                None
            },
            ModuleType::Conjunction(state) => {
                state[sourceno] = pulse;
                let output_pulse = if state.iter().all(|v| v == &Pulse::High) { Pulse::Low } else { Pulse::High };
                Some(output_pulse)
            },
            ModuleType::Output => None
        }
    }
}

struct Circuit {
    modules: HashMap<String, Module>,
    outputs: HashMap<String, Vec<String>>,
    inputs: HashMap<String, Vec<String>>
}

impl Circuit {
    fn push_button(&mut self) -> (usize, usize) {
        let mut q = VecDeque::<(&str, &str, Pulse)>::new();
        q.push_back(("", "broadcaster", Pulse::Low));
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        while !q.is_empty() {
            let (source, target, pulse) = q.pop_front().unwrap();
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1
            }
            match self.modules.get_mut(target).unwrap().handle_pulse(self.inputs.get(target).unwrap().iter().position(|x| x == source).unwrap(), pulse) {
                None => {},
                Some(output) => {
                    for next in self.outputs.get(target).unwrap() {
                        q.push_back((target, next, output));
                    }
                }
            }
        }

        (low_pulses, high_pulses)
    }

    #[allow(unused)]
    fn mermaid_graph(&self) -> String {
        "graph LR\n".to_owned() + self.modules.iter().map(|(name, module)| {
            let def: String = match module.module_type {
                ModuleType::Broadcaster | ModuleType::Output => format!("{name}[{name}]"),
                ModuleType::Conjunction(_) => format!("{name}{{{name}}}"),
                ModuleType::FlipFlop(_) => format!("{name}({name})")
            } + "\n";
            let connections: String = self.outputs.get(name).map(|x| x.iter().map(|t| format!("\t{name} --> {t}")).intersperse("\n\t".to_owned()).collect()).unwrap_or("".to_owned());
            def + &connections
        }).intersperse("\n".to_owned()).collect::<String>().as_ref()
    }
}

impl FromStr for Circuit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules = HashMap::new();
        let mut outputs = HashMap::new();
        let mut inputs = HashMap::new();

        modules.insert("output".to_owned(), Module { name: "output".to_owned(), module_type: ModuleType::Output });
        modules.insert("rx".to_owned(), Module { name: "rx".to_owned(), module_type: ModuleType::Output });
        inputs.insert("broadcaster".to_owned(), vec!["".to_owned()]);

        for line in s.lines() {
            let (source, targets) = line.split_once(" -> ").ok_or(Error::msg("Invalid line format"))?;
            let source = source.parse::<Module>()?;
            let targets: Vec<String> = targets.split(", ").map(|t| t.to_owned()).collect();

            for target in targets.iter() {
                if !inputs.contains_key(target) {
                    inputs.insert(target.to_owned(), Vec::new());
                }

                inputs.get_mut(target).unwrap().push(source.name.clone());
            }

            outputs.insert(source.name.clone(), targets);
            modules.insert(source.name.clone(), source);
        }

        for module in modules.values_mut().filter(|x| if let ModuleType::Conjunction(_) = x.module_type { true } else { false }) {
            let ModuleType::Conjunction(v) = &mut module.module_type else { panic!("will never happen") };
            for _ in 0..inputs.get(&module.name).unwrap().len() {
                v.push(Pulse::Low);
            }
        }

        Ok(Circuit { modules, outputs, inputs })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part1() {
        let mut circuit = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&mut circuit), 32000000);
    }

    const EXAMPLE_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
}