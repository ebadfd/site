/*
* language semantics.
*
* only allow defined commands to run;
* - whoami
* - cd
* - cat
* - echo
*
* Optional commands;
* - for loops
*
* Comment semantics are #
*/
#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    pub cur: usize,
    pub bol: usize,
    pub row: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            cur: 0,
            bol: 0,
            row: 0,
        }
    }

    pub fn has_char(&self) -> bool {
        return self.cur < self.source.len();
    }

    pub fn is_eof(&self) -> bool {
        return !self.has_char();
    }

    pub fn trim_left(&mut self) {
        while self.has_char() && self.source.chars().nth(self.cur).unwrap().is_whitespace() {
            self.chop_char();
        }
    }

    pub fn chop_char(&mut self) {
        if self.has_char() {
            let x = self
                .source
                .chars()
                .nth(self.cur.try_into().unwrap())
                .unwrap();
            self.cur += 1;

            if x == 0xA as char {
                self.bol = self.cur;
                self.row += 1
            }
        }
    }

    fn drop_line(&mut self) {
        while self.has_char() && self.source.chars().nth(self.cur).unwrap() != 0xA as char {
            self.chop_char();
        }

        if self.has_char() {
            self.chop_char();
        }
    }

    pub fn next_token(&mut self) {
        self.trim_left();
        let sub_str_current: String = self
            .source
            .clone()
            .chars()
            .into_iter()
            .skip(self.cur)
            .collect();

        if sub_str_current.starts_with("#") {
            self.drop_line();
            self.trim_left();
        }

        println!("Ok");
    }
}
