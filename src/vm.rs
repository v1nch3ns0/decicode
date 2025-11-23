
pub struct VM {
    pub stack: Vec<i32>,
    pub ip: usize,
    pub program: Vec<char>,
}

impl VM {
    pub fn new(code: &str) -> Self {
        let code = if code.starts_with("#!") {
            match code.find('\n') {
                Some(idx) => &code[idx + 1..],
                None => "",
            }
        } else {
            code
        };
        Self {
            stack: Vec::new(),
            ip: 0,
            program: code.chars().collect(),
        }
    }

    fn push(&mut self, val: i32) { self.stack.push(val); }

    fn pop(&mut self) -> i32 { self.stack.pop().expect("stack underflow") }

    fn peek(&self) -> i32 { *self.stack.last().expect("stack empty") }

    fn dup(&mut self) { let val = self.peek(); self.push(val); }

    fn swap(&mut self) {
        if self.stack.len() < 2 { panic!("not enough elements to swap"); }
        let len = self.stack.len();
        self.stack.swap(len - 1, len - 2);
    }

    fn read_literal(&mut self) -> usize {
        let mut literal = String::new();
        while self.ip + 1 < self.program.len() && self.program[self.ip + 1].is_digit(10) {
            self.ip += 1;
            literal.push(self.program[self.ip]);
        }
        literal.parse().expect("invalid literal")
    }

    pub fn step(&mut self) {
        // skip whitespace
        while self.ip < self.program.len() && self.program[self.ip].is_whitespace() {
            self.ip += 1;
        }
        
        if self.ip >= self.program.len() { return; }

        let opcode = self.program[self.ip];
        match opcode {
            '>' => {
                let mut literal = String::new();
                self.ip += 1;
                while self.ip < self.program.len() && self.program[self.ip].is_digit(10) {
                    literal.push(self.program[self.ip]);
                    self.ip += 1;
                }
                self.push(literal.parse().expect("invalid literal"));
                return;
            }
            '!' => { self.pop(); }
            '+' => { let a = self.pop(); let b = self.pop(); self.push(b + a); }
            '-' => { let a = self.pop(); let b = self.pop(); self.push(b - a); }
            '*' => { let a = self.pop(); let b = self.pop(); self.push(b * a); }
            '/' => { let a = self.pop(); let b = self.pop(); self.push(b / a); }
            '&' => self.dup(),
            '^' => { print!("{}", self.peek() ); }
            '@' => { print!("{}", self.peek() as u8 as char ); }
            '#' => {}
            '?' => {
                let target = self.read_literal();
                if self.pop() == 0 { self.ip = target; return; }
            }
            ':' => { self.ip = self.read_literal(); return; }
            '~' => { self.swap(); }

            _ => panic!("unknown opcode: {}", opcode),
        }

        self.ip += 1;
    }

    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            self.step();
        }
    }
}
