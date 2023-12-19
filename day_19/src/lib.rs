use std::{collections::HashMap, str::FromStr};

use anyhow::Error;

pub fn solve(input: String) {
    let (mut system, parts) = parse_input(&input);

    println!("{}", part1(&system, &parts));
    println!("{}", part2(&mut system));
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

fn part2(system: &mut System) -> usize {
    system.eval_workflow("in").iter().map(|pr| pr.elements()).sum()
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
    LT(usize),
    GT(usize)
}

impl Comparison {
    fn test(&self, lhs: usize) -> bool {
        match self {
            Self::LT(rhs) => lhs < *rhs,
            Self::GT(rhs) => lhs > *rhs
        }
    }

    fn inverted(&self) -> Self {
        match self {
            Self::LT(rhs) => Self::GT(rhs - 1),
            Self::GT(rhs) => Self::LT(rhs + 1)
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
    Conditional(RatingCategory, Comparison, RuleTarget),
    Unconditional(RuleTarget)
}

impl Rule {
    fn eval(&self, part: &Part) -> Option<RuleTarget> {
        match self {
            Self::Conditional(rc, cmp, target) => {
                if cmp.test(rc.get(part)) {
                    Some(target.clone())
                } else {
                    None
                }
            },
            Self::Unconditional(target) => Some(target.clone())
        }
    }

    fn fallthrough_range(&self) -> PartRange {
        if let Self::Conditional(cat, cmp, _) = self {
            return <PartRange as Default>::default().bound(cat, &cmp.inverted()).unwrap();
        }

        panic!("No fallthrough range for unconditional rule");
    }

    fn eval_range(&self, system: &mut System) -> Vec<PartRange> {
        match self {
            Self::Conditional(cat, cmp, target) => {
                match target {
                    RuleTarget::Accept => match <PartRange as Default>::default().bound(cat, cmp) {
                        Some(x) => vec![x],
                        None => vec![]
                    },
                    RuleTarget::Reject => vec![],
                    RuleTarget::Next(name) => {
                        system.eval_workflow(name).iter().filter_map(|pr| pr.bound(cat, cmp)).collect()
                    }
                }
            },
            Self::Unconditional(target) => {
                match target {
                    RuleTarget::Accept => vec![Default::default()],
                    RuleTarget::Reject => vec![],
                    RuleTarget::Next(name) => system.eval_workflow(name).clone(),
                }
            }
        }
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cond, target)) = s.split_once(":") {
            if let Some((cat, val)) = cond.split_once("<") {
                Ok(Self::Conditional(cat.parse()?, Comparison::LT(val.parse()?), target.parse()?))
            } else if let Some((cat, val)) = cond.split_once(">") {
                Ok(Self::Conditional(cat.parse()?, Comparison::GT(val.parse()?), target.parse()?))
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

    fn eval_range(&self, system: &mut System) -> Vec<PartRange> {
        self.rules.iter().scan((<PartRange as Default>::default(), true), |(state, should_continue), rule| {
            if !*should_continue {
                return None;
            }

            let result = rule.eval_range(system).iter().filter_map(|x| x.intersection(state)).collect::<Vec<_>>();

            if let Rule::Unconditional(_) = rule {
                *should_continue = false;
            } else {
                let new_state = state.intersection(&rule.fallthrough_range());

                if let Some(st) = new_state {
                    *state = st
                } else {
                    *should_continue = false;
                }
            }

            Some(result)
        }).flatten().collect()
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
    workflows: HashMap<String, Workflow>,
    evaluations: HashMap<String, Vec<PartRange>>
}

impl System {
    fn new(workflows: HashMap<String, Workflow>) -> Self {
        Self { workflows, evaluations: HashMap::new() }
    }
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

        Ok(System::new(workflows))
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

    fn eval_workflow(&mut self, name: &str) -> &Vec<PartRange> {
        if self.evaluations.get(name).is_none() {
            let workflow = self.workflows.get(name).unwrap().clone();
            let new_evaluation = workflow.eval_range(self);
            self.evaluations.insert(name.to_owned(), new_evaluation);
        }

        self.evaluations.get(name).unwrap()
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct PartRange {
    xrange: Range,
    mrange: Range,
    arange: Range,
    srange: Range
}

impl PartRange {
    fn intersection(&self, other: &PartRange) -> Option<PartRange> {
        let xrange = self.xrange.intersection(&other.xrange)?;
        let mrange = self.mrange.intersection(&other.mrange)?;
        let arange = self.arange.intersection(&other.arange)?;
        let srange = self.srange.intersection(&other.srange)?;

        Some(PartRange { xrange, mrange, arange, srange })
    }

    fn bound(&self, cat: &RatingCategory, cmp: &Comparison) -> Option<Self> {
        match cat {
            RatingCategory::X => Some(Self { xrange: self.xrange.intersection(&Range::from(cmp))?, ..*self }),
            RatingCategory::M => Some(Self { mrange: self.mrange.intersection(&Range::from(cmp))?, ..*self }),
            RatingCategory::A => Some(Self { arange: self.arange.intersection(&Range::from(cmp))?, ..*self }),
            RatingCategory::S => Some(Self { srange: self.srange.intersection(&Range::from(cmp))?, ..*self }),
        }
    }

    fn elements(&self) -> usize {
        self.xrange.elements() * self.mrange.elements() * self.arange.elements() * self.srange.elements()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Range {
    min: usize,
    max: usize
}

impl Range {
    fn intersection(&self, other: &Range) -> Option<Range> {
        if self.min > other.max || other.min > self.max {
            None
        } else {
            Some(Range { min: self.min.max(other.min), max: self.max.min(other.max) })
        }
    }

    fn elements(&self) -> usize {
        self.max - self.min + 1
    }
}

impl Default for Range {
    fn default() -> Self {
        Self { min: 1, max: 4000 }
    }
}

impl From<&Comparison> for Range {
    fn from(value: &Comparison) -> Self {
        match value {
            Comparison::GT(a) => Self { min: a + 1, ..Default::default() },
            Comparison::LT(b) => Self { max: b - 1, ..Default::default() }
        }
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

    #[test]
    fn example_part2() {
        let (mut system, _) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part2(&mut system), 167409079868000);
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