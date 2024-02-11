use crate::compiler::compile_functions::{Line, UserFunction};

pub struct Output {
    inner: String
}

impl Output {
    pub fn new() -> Output {
        Output { inner: String::new() }
    }

    pub fn new_with_name(id: isize) -> Output {
        Output { inner: format!("{}:\n", get_function_name(id)) }
    }


    pub fn push(&mut self, string: &str) {
        self.inner.push('\t');
        self.inner += string;
        self.inner.push('\n');
    }
}

impl From<Output> for String {
    fn from(value: Output) -> Self {
        value.inner
    }
}

pub fn get_function_name(id: isize) -> String {
    if id == 0 { return "main".to_string() }
    let sign = if id < 0 {
        "__"
    }
    else {
        "_"
    };
    format!("{sign}{}", id.abs())
}

pub fn get_function_sublabel(id: isize, label: &str) -> String {
    let mut base = if id == 0 { "main".to_string() }
    else {
        let sign = if id < 0 {
            "_"
        }
        else {
            ""
        };
        format!(".{sign}{}", id.abs())
    };

    base.push('.');
    base += label;
    base
}

pub fn get_local_address(addr: isize) -> String {
    let sign = if addr >= 0 {
        "+"
    }
    else {
        ""
    };
    format!("rbp{sign}{addr}")
}

pub fn compile_user_function(function: &UserFunction) -> String {
    let mut output = Output::new_with_name(function.id);
    output.push("push rbp");
    output.push("mov rbp, rsp");
    output.push(&format!("sub rsp, {}", function.local_variable_count * 8));

    for line in &function.lines {
        match line {
            Line::ReturnCall(function, local_args, return_addr) => {
                // Push args to stack
                for (local_addr, size) in local_args.iter().rev() {
                    output.push(&format!("mov rax, qword [{}]", get_local_address(*local_addr)));
                    output.push("push rax");
                }
                // Call
                output.push(&format!("call {}", get_function_name(*function)));

                // Release stack space used
                if !local_args.is_empty() {
                    output.push(&format!("add rsp, {}", local_args.len() * 8));
                }
                // Move return value
                output.push(&format!("mov qword [{}], rax", get_local_address(*return_addr)));
            }
            Line::NoReturnCall(function, local_args) => {
                // Push args to stack
                for (local_addr, size) in local_args.iter().rev() {
                    output.push(&format!("mov rax, qword [{}]", get_local_address(*local_addr)));
                    output.push("push rax");
                }
                // Call
                output.push(&format!("call {}", get_function_name(*function)));

                // Release stack space used
                output.push(&format!("add rsp, {}", local_args.len() * 8));
            }
            Line::Copy(local_from, local_to) => {
                output.push(&format!("mov rax, qword [{}]", get_local_address(*local_from)));
                output.push(&format!("mov qword [{}], rax", get_local_address(*local_to)));
            }
            Line::Return(local_return_val) => {
                return if function.id == 0 {
                    output.push(&format!("mov rcx, [{}]", get_local_address(local_return_val.unwrap())));
                    output.push("call ExitProcess");
                    output.into()
                } else {
                    if let Some(val) = local_return_val {
                        output.push(&format!("mov rax, [{}]", get_local_address(*val)));
                    }
                    output.push("leave");
                    output.push("ret");
                    output.into()
                }
            }
            Line::InlineAsm(asm) => {
                for line in asm {
                    output.push(line);
                }
            }
        }
    }

    return if function.id == 0 {
        output.push("mov rcx, 0");
        output.push("call ExitProcess");
        output.into()
    } else {
        output.push("leave");
        output.push("ret");
        output.into()
    }
}
