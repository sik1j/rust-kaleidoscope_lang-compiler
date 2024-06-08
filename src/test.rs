use std::io::Read;
use std::str::FromStr;


pub struct TestParser {
    pub next_char: char
}

impl TestParser {
    pub fn new() -> TestParser {
        TestParser {
            next_char: Self::get_char(),
        }
    }
    fn parse(&mut self) -> f64 {
        self.parse_primary()
    }


    // gets next non-whitespace char
    fn scan_next(&mut self) {
        loop {
            self.next_char = Self::get_char();
            if !self.next_char.is_whitespace() {
                break;
            }
        }
    }
    
    fn get_char() -> char {
        let mut buffer = [0;1];
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        handle.read_exact(&mut buffer).map_or(0 as char, |_| {buffer[0] as char})
    }
    pub fn parse_term(&mut self) -> f64 {
        let mut lhs = self.parse_factor();

        loop {
            let op = self.next_char;
            if op == '+' || op == '-' {
                self.scan_next();
                let rhs = self.parse_factor();
                lhs = match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    _ => panic!("what!"),
                }
            } else {
                break;
            }
        }
        lhs
    }

    pub fn parse_factor(&mut self) -> f64 {
        let mut lhs = self.parse_primary();

        loop {
            let op = self.next_char;
            if op == '*' || op == '/' {
                self.scan_next();
                let rhs = self.parse_primary();
                lhs = match op {
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    _ => panic!("what!"),
                }
            } else {
                break;
            }
        }
        lhs
    }
    pub fn parse_num(&mut self) -> f64 {
        let mut res = String::from(self.next_char);

        loop {
            self.next_char = Self::get_char();
            if self.next_char.is_digit(10) {
                res.push(self.next_char);
            } else {
                break;
            }
        }

        if self.next_char.is_whitespace() {
            self.scan_next();
        }

        f64::from_str(res.as_str()).unwrap()
    }
    fn parse_primary(&mut self) -> f64 {
        self.parse_num()
    }
}

pub struct LoopParser {
    pub next_char: char
}

impl LoopParser {
    pub fn new() -> Self {
        LoopParser {
            next_char: Self::get_char(),
        }
    }

    pub fn parse_num(&mut self) -> f64 {
        let mut res = String::from(self.next_char);

        loop {
            self.next_char = Self::get_char();
            if self.next_char.is_digit(10) {
                res.push(self.next_char);
            } else {
                break;
            }
        }

        if self.next_char.is_whitespace() {
            self.scan_next();
        }

        f64::from_str(res.as_str()).unwrap()
    }

    fn get_char() -> char {
        let mut buffer = [0;1];
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        handle.read_exact(&mut buffer).map_or(0 as char, |_| {buffer[0] as char})
    }

    fn scan_next(&mut self) {
        loop {
            self.next_char = Self::get_char();
            if !self.next_char.is_whitespace() {
                break;
            }
        }
    }

    fn get_prec(c: char) -> i64 {
        match c {
            '+' => 10,
            '*' => 40,
            _ => -1,
        }
    }

    pub fn parse_biop(&mut self, mut lhs: f64, min_prec: i64) -> f64 {
        loop {
            let op =  self.next_char;
            let cur_prec = Self::get_prec(op);
            print!("cur op: {},", op);
            if  cur_prec < min_prec  {
                return lhs;
            }

            self.scan_next(); // consume op
            let mut rhs = self.parse_primary();
            let next_op = self.next_char;
            println!("next op: {},", next_op);

            let next_prec = Self::get_prec(next_op);

            println!("cp, np: {}, {}", cur_prec, next_prec);

            if next_prec > cur_prec { // if next op binds tighter, eval next first
                rhs = self.parse_biop(rhs, cur_prec+1);
            }

            lhs = match op {
                '+' => lhs + rhs,
                '*' => lhs * rhs,
                _ => panic!("Unknown op"),
            }
        }
    }
    pub fn parse_primary(&mut self) -> f64 {
        match self.next_char {
            '(' =>  {
                self.scan_next(); // consume '('
                let lhs = self.parse_primary();
                let val = self.parse_biop(lhs, 0);
                if self.next_char != ')' {
                    panic!("expected closing ')'");
                }
                self.scan_next();
                return val;
            }
            _ => self.parse_num()
        }
    }
}

pub struct LoopParserString {
    pub next_char: char
}

impl LoopParserString {
    pub fn new() -> Self {
        LoopParserString {
            next_char: Self::get_char(),
        }
    }

    pub fn parse_num(&mut self) -> String {
        let mut res = String::from(self.next_char);

        loop {
            self.next_char = Self::get_char();
            if self.next_char.is_digit(10) {
                res.push(self.next_char);
            } else {
                break;
            }
        }

        if self.next_char.is_whitespace() {
            self.scan_next();
        }

        res
    }

    fn get_char() -> char {
        let mut buffer = [0;1];
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        handle.read_exact(&mut buffer).map_or(0 as char, |_| {buffer[0] as char})
    }

    fn scan_next(&mut self) {
        loop {
            self.next_char = Self::get_char();
            if !self.next_char.is_whitespace() {
                break;
            }
        }
    }

    fn get_prec(c: char) -> i64 {
        match c {
            '+' => 10,
            '-' => 10,
            '/' => 40,
            '*' => 40,
            _ => -1,
        }
    }

    pub fn parse_biop(&mut self, mut lhs: String, min_prec: i64) -> String {
        loop {
            let op =  self.next_char;
            let cur_prec = Self::get_prec(op);
            print!("cur op: {},", op);
            if  cur_prec < min_prec  {
                return format!("{}", lhs);
            }

            self.scan_next(); // consume op
            let mut rhs = self.parse_primary();
            let next_op = self.next_char;
            println!("next op: {},", next_op);

            let next_prec = Self::get_prec(next_op);

            println!("cp, np: {}, {}", cur_prec, next_prec);

            if next_prec > cur_prec { // if next op binds tighter, eval next first
                rhs = self.parse_biop(rhs, cur_prec+1);
            }

            lhs = format!("({}{}{})", lhs, op, rhs)
        }
    }
    pub fn parse_primary(&mut self) -> String {
        match self.next_char {
            '(' =>  {
                self.scan_next(); // consume '('
                let lhs = self.parse_primary();
                let val = self.parse_biop(lhs, 0);
                if self.next_char != ')' {
                    panic!("expected closing ')'");
                }
                self.scan_next();
                return val;
            }
            _ => self.parse_num()
        }
    }
}
