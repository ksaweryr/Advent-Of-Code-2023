use std::{collections::HashMap, str::FromStr};

use anyhow::Error;

pub fn solve(input: String) {
    let (system, parts) = parse_input(&input);

    println!("{}", part1(&system, &parts));
}

fn parse_input(input: &str) -> (System, Vec<Part>) {
    let (system, parts) = input.split_once("\n\n").expect("Invalid input format");

    let system = system.parse().expect("Couldn't parse system");
    let parts = parts.lines().map(|l| l.parse::<Part>()).collect::<Result<Vec<Part>, _>>().expect("Couldn't parse parts");

    (system, parts)
}

fn part1(system: &System, parts: &Vec<Part>) -> usize {
    parts.iter().filter(|p| system.is_accepted(*p)).map(|p| p.value()).sum()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RatingCategory {
    X, M, A, S
}

impl FromStr for RatingCategory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "x" {
            Ok(Self::X)
        } else if s == "m" {
            Ok(Self::M)
        } else if s == "a" {
            Ok(Self::A)
        } else if s == "s" {
            Ok(Self::S)
        } else {
            Err(Error::msg(format!("Invalid category: `{}`", s)))
        }
    }
}

impl RatingCategory {
    fn get(&self, part: &Part) -> usize {
        match self {
            Self::X => part.x,
            Self::M => part.m,
            Self::A => part.a,
            Self::S => part.s
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Comparison {
    LT,
    GT
}

impl Comparison {
    fn test(&self, lhs: usize, rhs: usize) -> bool {
        match self {
            Self::LT => lhs < rhs,
            Self::GT => lhs > rhs
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum RuleTarget {
    Accept,
    Reject,
    Next(String)
}

impl FromStr for RuleTarget {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "A" {
            Ok(Self::Accept)
        } else if s == "R" {
            Ok(Self::Reject)
        } else {
            Ok(Self::Next(s.to_owned()))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Rule {
    Conditional(RatingCategory, Comparison, usize, RuleTarget),
    Unconditional(RuleTarget)
}

impl Rule {
    fn eval(&self, part: &Part) -> Option<RuleTarget> {
        match self {
            Self::Conditional(rc, cmp, value, target) => {
                if cmp.test(rc.get(part), *value) {
                    Some(target.clone())
                } else {
                    None
                }
            },
            Self::Unconditional(target) => Some(target.clone())
        }
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cond, target)) = s.split_once(":") {
            if let Some((cat, val)) = cond.split_once("<") {
                Ok(Self::Conditional(cat.parse()?, Comparison::LT, val.parse()?, target.parse()?))
            } else if let Some((cat, val)) = cond.split_once(">") {
                Ok(Self::Conditional(cat.parse()?, Comparison::GT, val.parse()?, target.parse()?))
            } else {
                Err(Error::msg(format!("Invalid operator in `{}`", cond)))
            }
        } else {
            Ok(Self::Unconditional(s.parse()?))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>
}

impl Workflow {
    fn eval(&self, part: &Part) -> RuleTarget {
        self.rules.iter().find_map(|r| r.eval(part)).expect("No rule matched")
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = s.split_once("{").ok_or(Error::msg("Invalid workflow format"))?;
        let name = name.to_owned();
        let rules = rules[..rules.len()-1].split(",").map(|r| r.parse::<Rule>()).collect::<Result<Vec<Rule>, _>>()?;

        Ok(Workflow { name, rules })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct System {
    workflows: HashMap<String, Workflow>
}

impl FromStr for System {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let workflows = s.lines().map(|l| l.parse::<Workflow>()).map(|r| {
            match r {
                Ok(w) => Ok((w.name.clone(), w)),
                Err(msg) => Err(msg)
            }
        }).collect::<Result<HashMap<String, Workflow>, _>>()?;

        Ok(System { workflows })
    }
}

impl System {
    fn is_accepted(&self, part: &Part) -> bool {
        self._is_accepted(part, RuleTarget::Next("in".to_owned()))
    }

    fn _is_accepted(&self, part: &Part, target: RuleTarget) -> bool {
        match target {
            RuleTarget::Accept => true,
            RuleTarget::Reject => false,
            RuleTarget::Next(name) => self._is_accepted(part, self.workflows.get(&name).unwrap().eval(part))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

impl Part {
    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, m, a, s] = s[1..s.len()-1].split(",")
            .map(|x| x.split_once("=")
                .ok_or(Error::msg("Invalid part format"))
                .and_then(|(_, v)| v.parse::<usize>().map_err(Error::new)))
            .collect::<Result<Vec<usize>, _>>()?.try_into().map_err(|_| Error::msg("Invalid part format"))?;

        Ok(Part { x, m, a, s })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_part() {
        let part = "{x=787,m=2655,a=1222,s=2876}".parse::<Part>().unwrap();

        assert_eq!(part, Part { x: 787, m: 2655, a: 1222, s: 2876 });
    }

    #[test]
    fn example_part1() {
        let (system, parts) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&system, &parts), 19114);
    }

    const EXAMPLE_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
}