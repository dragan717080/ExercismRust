use std::collections::HashMap;
use std::result::Result as StdResult;

fn main() {
    let mut f = Forth::new();
    let _ = f.eval("1 2 3 swap");
    assert_eq!(f.stack(), [1, 3, 2]);

    let mut f = Forth::new();
    assert!(f.eval(": foo 5 ;").is_ok());
    assert!(f.eval(": bar foo ;").is_ok());
    assert!(f.eval(": foo 6 ;").is_ok());
    assert!(f.eval("bar foo").is_ok());
    assert_eq!(f.stack(), [5, 6]);
}

pub type Value = i32;
pub type Result<T> = StdResult<T, Error>;

pub struct Forth {
    elements: Vec<Value>,
    commands: HashMap<String, Vec<String>>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        Self { elements: Vec::new(), commands: HashMap::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.elements[..]
    }

    pub fn eval(&mut self, input: &str) -> Result<()> {
        let input = input.to_lowercase();
        let words = input.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        if let Some(first_word) = words.get(0) {
            if *first_word == ":" {
                return Ok(self.make_custom_command(&words[1..words.len() - 1])?);
            }
        }

        for word in words {
            match word.parse::<Value>() {
                Ok(n) => self.elements.push(n),
                Err(_) => {
                    let command = word.as_ref();

                    if let Some(custom_subcommands) = self.commands.get(command).map(|v| v.clone()) {
                        let subcommands = custom_subcommands.iter().map(|command| command.as_str()).collect::<Vec<&str>>();
                        let subcommand_words = subcommands.join(" ");
                        let _ = self.eval(&subcommand_words.as_str());

                        continue;
                    }

                    let _ = match command {
                        "dup" => self.handle_duplicate()?,
                        "drop" => self.handle_drop()?,
                        "swap" => self.handle_swap()?,
                        "over" => self.handle_over()?,
                        "*" => self.handle_multiplication()?,
                        "/" => self.handle_division()?,
                        "+" => self.handle_addition()?,
                        "-" => self.handle_substraction()?,
                        _ => { return Err(Error::UnknownWord); }
                    };
                }
            }
        }

        Ok(())
    }

    fn find_last_element(&self) -> std::result::Result<&i32, Error> {
        self.elements.last()
        .ok_or(Error::StackUnderflow)
        .map(|last| last)
    }

    fn find_last_two_elements(&self) -> std::result::Result<(&i32, &i32), Error> {
        let n = self.elements.len();
        n.checked_sub(2)
            .and_then(|idx| self.elements.get(idx))
            .ok_or(Error::StackUnderflow)
            .and_then(|second_to_last| {
                self.elements.last()
                    .ok_or(Error::StackUnderflow)
                    .map(|last| (second_to_last, last))
            })
    }

    fn handle_duplicate(&mut self) -> Result<()> {
        let last = self.find_last_element()?;

        Ok(self.elements.push(*last))
    }

    fn handle_drop(&mut self) -> Result<()> {
        self.find_last_element()?;
        self.elements.pop().unwrap();

        Ok(())
    }

    fn handle_swap(&mut self) -> Result<()> {
        let n = self.elements.len();
        self.find_last_two_elements()?;

        Ok(self.elements.swap(n - 1, n - 2))
    }

    fn handle_over(&mut self) -> Result<()> {
        let (second_to_last, _) = self.find_last_two_elements()?;
        
        Ok(self.elements.push(*second_to_last))
    }

    fn handle_multiplication(&mut self) -> Result<()> {
        let (second_to_last, last) = self.find_last_two_elements()?;

        Ok(self.add_arithmetic(second_to_last * last))
    }

    fn handle_division(&mut self) -> Result<()> {
        let (second_to_last, last) = self.find_last_two_elements()?;

        if last == &0 {
            return Err(Error::DivisionByZero);
        }

        Ok(self.add_arithmetic(second_to_last / last))
    }

    fn handle_addition(&mut self) -> Result<()> {
        let (second_to_last, last) = self.find_last_two_elements()?;

        Ok(self.add_arithmetic(second_to_last + last))
    }

    fn handle_substraction(&mut self) -> Result<()> {
        let (second_to_last, last) = self.find_last_two_elements()?;

        Ok(self.add_arithmetic(second_to_last - last))
    }

    /**
     * Removes last two elements and pushes the arithmetic result.
     */
    fn add_arithmetic(&mut self, result: Value) {
        self.elements.pop();
        self.elements.pop();
        self.elements.push(result);
    }

    fn make_custom_command(&mut self, subcommands: &[String]) -> Result<()> {
        let (command, subcommands) = subcommands.split_at(1);
        let command = &command[0];

        if command.parse::<i64>().is_ok() {
            return Err(Error::InvalidWord);
        }

        // Handle the case where one custom variable is dependent on another, and first one changes,
        // then second variable shouldn't change
        for subcommand in subcommands {
            if command == subcommand {
                continue;
            }

            if let Some(custom_subcommands) = self.commands.get(subcommand).map(|v| v.clone()) {
                self.commands.insert(command.to_string(), custom_subcommands);

                return Ok(());
            }
        }

        self.commands.insert(command.to_string(), subcommands.to_vec());

        Ok(())
    }
}
