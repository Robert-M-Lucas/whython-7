use crate::compiler::compile_functions::{Line, UserFunction};

pub struct Output {
    inner: String,
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

impl Output {
    pub fn new() -> Output {
        Output {
            inner: String::new(),
        }
    }

    pub fn new_with_name(id: isize, name: &str) -> Output {
        Output {
            inner: format!("{}: ; {}\n", get_function_name(id), name),
        }
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
    if id == 0 {
        return "main".to_string();
    }
    let sign = if id < 0 { "__" } else { "_" };
    format!("{sign}{}", id.abs())
}

pub fn get_function_sublabel(id: isize, label: &str) -> String {
    let mut base = if id == 0 {
        "main".to_string()
    } else {
        let sign = if id < 0 { "_" } else { "" };
        format!(".{sign}{}", id.abs())
    };

    base.push('.');
    base += label;
    base
}

pub fn get_local_address(addr: isize) -> String {
    let sign = if addr >= 0 { "+" } else { "" };
    format!("rbp{sign}{addr}")
}

pub fn compile_user_function(function: &UserFunction) -> String {
    let mut output = Output::new_with_name(function.id, &function.name);
    output.push("push rbp");
    output.push("mov rbp, rsp");
    output.push(&format!(
        "sub rsp, {}",
        (function.local_variable_count * 8) + (function.local_variable_count % 2) * 8
    ));

    let mut last_return = false;
    for line in &function.lines {
        last_return = false;
        match line {
            Line::ReturnCall(function, local_args, return_addr) => {
                if local_args.len() % 2 != 0 {
                    output.push("push qword 0");
                }
                // Push args to stack
                for (local_addr, _size) in local_args.iter().rev() {
                    let mut local_addr = *local_addr;
                    let mut size = *_size as isize;
                    local_addr += size;
                    local_addr -= 8;
                    while size > 0 {
                        output.push(&format!(
                            "mov rax, qword [{}]",
                            get_local_address(local_addr)
                        ));
                        output.push("push rax");
                        local_addr -= 8;
                        size -= 8;
                    }
                }
                // Call
                output.push(&format!("call {}", get_function_name(*function)));

                // Release stack space used
                if !local_args.is_empty() {
                    output.push(&format!("add rsp, {}", local_args.len() * 8 + (local_args.len() % 2) * 8));
                }
                // Move return value
                output.push(&format!(
                    "mov qword [{}], rax",
                    get_local_address(*return_addr)
                ));
            }
            Line::NoReturnCall(function, local_args) => {
                if local_args.len() % 2 != 0 {
                    output.push("push qword 0");
                }
                // Push args to stack
                for (local_addr, _size) in local_args.iter().rev() {
                    let mut local_addr = *local_addr;
                    let mut size = *_size as isize;
                    local_addr += size;
                    local_addr -= 8;
                    while size > 0 {
                        output.push(&format!(
                            "mov rax, qword [{}]",
                            get_local_address(local_addr)
                        ));
                        output.push("push rax");
                        local_addr -= 8;
                        size -= 8;
                    }
                }
                // Call
                output.push(&format!("call {}", get_function_name(*function)));

                // Release stack space used
                if !local_args.is_empty() {
                    output.push(&format!("add rsp, {}", local_args.len() * 8 + (local_args.len() % 2) * 8));
                }
            }
            Line::Copy(local_from, local_to, amount) => {
                let mut done = 0;
                while done < *amount {
                    output.push(&format!(
                        "mov rax, qword [{}]",
                        get_local_address(*local_from + (done as isize))
                    ));
                    output.push(&format!(
                        "mov qword [{}], rax",
                        get_local_address(*local_to + (done as isize))
                    ));
                    done += 8;
                }
            },
            Line::DynFromCopy(local_dyn_from, local_to, amount) => {
                let mut done = 0;
                output.push(&format!("mov r9, qword [{}]", get_local_address(*local_dyn_from)));
                while done < *amount {
                    output.push(&format!(
                        "mov rax, qword [r9+{}]",
                        done
                    ));
                    output.push(&format!(
                        "mov qword [{}], rax",
                        get_local_address(*local_to + (done as isize))
                    ));
                    done += 8;
                }
            },
            Line::DynToCopy(local_from, local_dyn_to, amount) => {
                let mut done = 0;
                output.push(&format!("mov r9, qword [{}]", get_local_address(*local_dyn_to)));
                while done < *amount {
                    output.push(&format!(
                        "mov rax, qword [{}]",
                        get_local_address(*local_from + (done as isize))
                    ));
                    output.push(&format!(
                        "mov qword [r9+{}], rax",
                        done
                    ));
                    done += 8;
                }
            },
            Line::Return(local_return_val) => {
                last_return = true;
                if function.id == 0 {
                    output.push(&format!(
                        "mov rcx, [{}]",
                        get_local_address(local_return_val.unwrap())
                    ));
                    output.push("call ExitProcess");
                } else {
                    if let Some(val) = local_return_val {
                        output.push(&format!("mov rax, [{}]", get_local_address(*val)));
                    }
                    output.push("leave");
                    output.push("ret");
                }
            },
            Line::HeapAlloc(amount, local_ref_addr) => {
                output.push("call GetProcessHeap"); // Get process heap
                output.push("mov rcx, rax"); // Heap handle
                output.push("mov rdx, rax"); // Flags
                output.push(&format!("mov r8, {}", *amount));
                output.push("call HeapAlloc");
                output.push(&format!("mov qword [{}], rax", get_local_address(*local_ref_addr)));
            },
            Line::InlineAsm(asm) => {
                for line in asm {
                    output.push(line);
                }
            }
            Line::Annotation(annotation) => {
                output.push(&format!("; '{}'", annotation));
            }
        }
    }

    if last_return {
        return output.into();
    }

    if function.id == 0 {
        output.push("mov rcx, 0");
        output.push("call ExitProcess");
        output.into()
    } else {
        output.push("leave");
        output.push("ret");
        output.into()
    }
}
