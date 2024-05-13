use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Instruction {
    Arithmetic(Op),
    Push(Segment, u16),
    Pop(Segment, u16),
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

#[derive(Debug, Clone)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let mut words = s.split_whitespace();

        // assume line is not empty
        let word = words.next().unwrap();

        if let Ok(op) = Op::from_str(word) {
            return Ok(Arithmetic(op));
        }

        match word {
            "push" | "pop" => {
                let segment = words.next().unwrap();
                use Segment::*;
                let segment = match segment {
                    "argument" => Ok(Argument),
                    "local" => Ok(Local),
                    "static" => Ok(Static),
                    "constant" => Ok(Constant),
                    "this" => Ok(This),
                    "that" => Ok(That),
                    "pointer" => Ok(Pointer),
                    "temp" => Ok(Temp),
                    _ => Err(format!("unable to parse segment: {segment} in line: {s}")),
                }?;

                let index = words.next().unwrap().parse().unwrap();

                match word {
                    "push" => Ok(Push(segment, index)),
                    "pop" => Ok(Pop(segment, index)),
                    _ => unreachable!(),
                }
            }

            _ => Err(format!("unable to parse line: {s}")),
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Op::*;
        match s {
            "add" => Ok(Add),
            "sub" => Ok(Sub),
            "neg" => Ok(Neg),
            "eq" => Ok(Eq),
            "gt" => Ok(Gt),
            "lt" => Ok(Lt),
            "and" => Ok(And),
            "or" => Ok(Or),
            "not" => Ok(Not),
            _ => Err(()),
        }
    }
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Segment::*;

        match s {
            "argument" => Ok(Argument),
            "local" => Ok(Local),
            "static" => Ok(Static),
            "constant" => Ok(Constant),
            "this" => Ok(This),
            "that" => Ok(That),
            "pointer" => Ok(Pointer),
            "temp" => Ok(Temp),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn parses_instructions() -> Result<(), Box<dyn Error>> {
        use Instruction::*;

        assert!(matches!(
            "push constant 112".parse()?,
            Push(Segment::Constant, 112)
        ));

        assert!(matches!("lt".parse()?, Arithmetic(Op::Lt)));

        Ok(())
    }
}
