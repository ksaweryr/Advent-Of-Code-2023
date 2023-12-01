use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::Chars;

struct Node {
    character: char,
    word: Option<String>,
    in_dict: bool,
    suffix: Option<Rc<RefCell<Node>>>,
    dict_suffix: Option<Rc<RefCell<Node>>>,
    children: HashMap<char, Rc<RefCell<Node>>>
}

impl Node {
    fn new(character: char, in_dict: bool, suffix: Option<Rc<RefCell<Node>>>, dict_suffix: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            character,
            word: None,
            in_dict,
            suffix,
            dict_suffix,
            children: HashMap::new()
        }))
    }
}

pub struct Match {
    pub position: usize,
    pub word: String
}

pub struct AhoCorasick {
    root: Rc<RefCell<Node>>
}

impl AhoCorasick {
    pub fn new() -> Self {
        AhoCorasick {
            root: Node::new(0 as char, false, None, None)
        }
    }

    pub fn from_words(words: Vec<String>) -> Self {
        let ac = Self::new();
        words.iter().for_each(|word| ac.add_word(word));
        ac.build();
        ac
    }

    pub fn add_word(&self, word: &str) {
        self._add_word(&mut word.chars(), word.to_owned(), self.root.clone(), word.len() - 1);
    }

    fn _add_word(&self, chars: &mut Chars<'_>, word: String, node: Rc<RefCell<Node>>, cnt: usize) {
        let char = match chars.next() {
            Some(c) => c,
            None => return
        };

        if !node.borrow().children.contains_key(&char) {
            node.borrow_mut().children.insert(char, Node::new(char, cnt == 0, None, None));
        }

        if cnt == 0 {
            node.borrow_mut().children.get(&char).unwrap().clone().borrow_mut().word = Some(word);
        } else {
            self._add_word(chars, word, node.borrow().children.get(&char).unwrap().clone(), cnt - 1);
        }
    }

    fn build(&self) {
        let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();

        for (_, child) in self.root.borrow().children.iter() {
            child.borrow_mut().suffix = Some(self.root.clone());
            child.borrow_mut().dict_suffix = Some(self.root.clone());
            q.push_back(child.clone());
        }

        while !q.is_empty() {
            let node = q.pop_front().unwrap();

            for (char, child) in node.borrow().children.iter() {
                let mut suffix = node.borrow().suffix.clone().unwrap_or_else(|| node.clone()).clone();
                
                while suffix.borrow().suffix.is_some() && !suffix.borrow().children.contains_key(char) {
                    let tmp = suffix.borrow().suffix.clone().unwrap().clone();
                    suffix = tmp;
                }

                if suffix.borrow().children.contains_key(char) {
                    let tmp = suffix.borrow().children.get(char).unwrap().clone();
                    suffix = tmp;
                }

                child.borrow_mut().suffix = Some(suffix);

                let mut dict_suffix = node.borrow().dict_suffix.clone().unwrap_or_else(|| node.clone()).clone();
                
                while dict_suffix.borrow().dict_suffix.is_some() && !(dict_suffix.borrow().children.contains_key(char) && dict_suffix.borrow().children.get(char).unwrap().borrow().in_dict) {
                    let tmp = dict_suffix.borrow().dict_suffix.clone().unwrap().clone();
                    dict_suffix = tmp;
                }

                if dict_suffix.borrow().children.contains_key(char) {
                    let tmp = dict_suffix.borrow().children.get(char).unwrap().clone();
                    dict_suffix = tmp;
                }

                child.borrow_mut().dict_suffix = Some(dict_suffix);

                q.push_back(child.clone());
            }
        }
    }

    pub fn find_matches(&self, word: &str) -> impl Iterator<Item=Match> {
        Iter {
            current_node: self.root.clone(),
            word: word.chars().collect::<Vec<char>>(),
            position: 0
        }
    }
}

struct Iter {
    current_node: Rc<RefCell<Node>>,
    word: Vec<char>,
    position: usize
}

impl Iterator for Iter {
    type Item = Match;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.position..self.word.len() {
            while self.current_node.borrow().character != 0 as char && !self.current_node.borrow().children.contains_key(&self.word[i]) {
                let tmp = self.current_node.borrow().suffix.clone().unwrap();
                self.current_node = tmp;
            }

            let tmp = self.current_node.borrow().children.get(&self.word[i]).unwrap_or(&self.current_node).clone();
            self.current_node = tmp;

            if self.current_node.borrow().in_dict {
                let word = self.current_node.borrow().word.clone().unwrap();
                let start = i - (word.len() - 1);
                self.position = i + 1;
                return Some(Match { position: start, word });
            }
        }

        self.position = self.word.len();
        return None;
    }
}