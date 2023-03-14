#[derive(PartialEq, Eq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    None
}

impl Operation {
    pub fn parse(string: &str) -> Self {
        match string {
            "plus" => Self::Add,
            "minus" => Self::Sub,
            "multiplied" => Self::Mul,
            "divided" => Self::Div,
            _ => Self::None
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let mut ans = 0;
    let mut cmd = (&command[..command.len() - 1]).split(' ').skip(2).enumerate();
    let mut operation = Operation::None;
    let mut skips = 0;
    let mut words = 0;

    while let Some((i, word)) = cmd.next() {
        words += 1;

        if (i + skips) % 2 == 0 {
            match word.parse::<i32>() {
                Ok(num) => match operation {
                    Operation::Sub => ans -= num,
                    Operation::Mul => ans *= num,
                    Operation::Div => ans /= num,
                    _ => ans += num
                },
                Err(_) => return None
            }
        } else {
            match Operation::parse(word) {
                Operation::None => return None,
                op if op == Operation::Div || op == Operation::Mul => {
                    operation = op;
                    cmd.next();
                    skips += 1;
                },
                op => operation = op
            }
        }
    }

    (words > 0 && words % 2 == 1)
        .then_some(ans)
}
