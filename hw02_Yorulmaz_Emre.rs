
#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn from_char(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' => Some(Op::Mul),
            '/' => Some(Op::Div),
            _ => None,
        }
    }

    fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    fn precedence(self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }

    fn apply(self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => {
                if b == 0.0 {
                    None
                } else {
                    Some(a / b)
                }
            }
        }
    }
}

fn evaluate_simple(a: f64, op: char, b: f64) -> Option<f64> {
    Op::from_char(op)?.apply(a, b)
}

#[derive(Debug)]
struct Stack {
    items: Vec<f64>,
}

impl Stack {
    fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    fn push(&mut self, v: f64) {
        self.items.push(v);
    }

    fn pop(&mut self) -> Option<f64> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&f64> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn apply(&mut self, op: Op) -> Option<()> {
        if self.items.len() < 2 {
            return None;
        }

        let len = self.items.len();
        let a = self.items[len - 2];
        let b = self.items[len - 1];

        let result = op.apply(a, b)?;

        self.items.pop();
        self.items.pop();
        self.items.push(result);

        Some(())
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(2.0);
    stack.push(3.0);
    stack.apply(Op::Add);

    stack.push(4.0);
    stack.apply(Op::Mul);

    stack.push(1.0);
    stack.apply(Op::Sub);

    while let Some(v) = stack.pop() {
        println!("{}", v);
    }
}
