pub struct Function {
    arguments: Vec<(String, isize)>,
    local_variables: Vec<(String, isize)>,
    return_type: Option<isize>,
}

impl Function {
    pub fn new(arguments: Vec<(String, isize)>, return_type: Option<isize>) -> Function {
        Function {
            arguments,
            local_variables: Vec::new(),
            return_type,
        }
    }

    pub fn add_variable(&mut self, name: String, type_: isize) -> usize {
        self.local_variables.push((name, type_));
        self.local_variables.len() - 1
    }

    pub fn get_variable(&self, name: &str) -> Result<(usize, isize), ()> {
        for i in 0..self.arguments.len() {
            if self.arguments[i].0 == name {
                return Ok((i, self.arguments[i].1));
            }
        }

        for i in 0..self.local_variables.len() {
            if self.local_variables[i].0 == name {
                return Ok((self.arguments.len() + i, self.local_variables[i].1));
            }
        }

        Err(())
    }
}
