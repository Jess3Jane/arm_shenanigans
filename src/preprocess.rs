use std::collections::HashMap;

pub struct Preprocessor {
    line_rules: HashMap<String, Vec<String>>,
    word_rules: HashMap<String, Vec<String>>,
}

impl Preprocessor {
    pub fn new() -> Preprocessor {
        Preprocessor { line_rules: HashMap::new(), word_rules: HashMap::new() }
    }

    pub fn add_line_rule(&mut self, rule: String, to: Vec<String>) {
        self.line_rules.insert(rule, to);
    }

    pub fn add_word_rule(&mut self, rule: String, to: Vec<String>) {
        self.word_rules.insert(rule, to);
    }

    fn process_word(&self, word: &str) -> Vec<String> {
        match self.word_rules.get(word) {
            Some(v) => {
                let mut ret = vec![];
                for word in v {
                    ret.append(&mut self.process_word(word));
                }
                ret
            },
            None => vec![String::from(word)],
        }
    }

    fn process_line(&self, line: &str) -> Vec<String> {
        match self.line_rules.get(line) {
            Some(v) => {
                let mut ret = vec![];
                for line in v {
                    ret.append(&mut self.process_line(line));
                }
                ret
            },
            None => vec![String::from(line)],
        }
    }

    pub fn preprocess(&self, text: &str) -> String {
        text.split('\n').map(|line| {
            self.process_line(line).into_iter().map(|line| {
                line.split_whitespace().map(|word| {
                    self.process_word(word).join(" ")
                }).collect::<Vec<String>>().join(" ")
            }).collect::<Vec<String>>().join("\n")
        }).collect::<Vec<String>>().join("\n")
    }
}
