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

    pub fn match_first(&self, text: &str) -> Option<(usize, String)> {
        let chars = text.chars();
        let mut node = self.root.clone();

        for (i, c) in chars.enumerate() {
            while node.borrow().character != 0 as char && !node.borrow().children.contains_key(&c) {
                let tmp = node.borrow().suffix.clone().unwrap();
                node = tmp;
            }

            let tmp = node.borrow().children.get(&c).unwrap_or(&node).clone();
            node = tmp;

            if node.borrow().in_dict {
                let word = node.borrow().word.clone().unwrap();
                let start = i - (word.len() - 1);
                return Some((start, word));
            }
        }

        return None;
    }
}