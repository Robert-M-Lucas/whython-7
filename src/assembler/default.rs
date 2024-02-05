use crate::processor::function_processor::{Line, UserFunction};

struct Output {
    inner: String
}

impl Output {
    pub fn new(id: isize) -> Output {
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
        "_"
    }
    else {
        ""
    };
    format!(".{sign}{}", id.abs())
}

fn get_local_address(addr: isize) -> String {
    let sign = if addr > 0 {
        "+"
    }
    else {
        ""
    };
    format!("rbp{sign}{addr}")
}

pub fn compile_user_function(function: &UserFunction) -> String {
    let mut output = Output::new(function.id);
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
                output.push(&format!("add rsp {}", local_args.len() * 8));
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
                output.push(&format!("add rsp {}", local_args.len() * 8));
            }
            Line::Copy(local_from, local_to) => {
                output.push(&format!("mov rax, qword [{}]", get_local_address(*local_from)));
                output.push(&format!("mov qword [{}], rax", get_local_address(*local_to)));
            }
            Line::Return(local_return_val) => {
                return if function.id == 0 {
                    output.push(&format!("mov rcx, [{}]", get_local_address(*local_return_val)));
                    output.push("call ExitProcess");
                    output.into()
                } else {
                    output.push(&format!("mov rax, [{}]", get_local_address(*local_return_val)));
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

    output.push("leave"); // redundant if rsp not changed (?)
    output.push("ret");
    output.into()
}
