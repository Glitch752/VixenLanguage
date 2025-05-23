use super::ast::{Declaration, Expression, LoopType, Program, Statement, StructElement, Type, VariableMutability};

pub struct ASTPrinter {
    indent: usize,
}

const ANSI_GRAY: &str = "\x1b[90m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RESET: &str = "\x1b[0m";

// A replacement for format! that includes indentation
macro_rules! fmt_indent {
    ($self:ident, $fmt:expr $(, $args:expr)*) => {{
        let mut output = String::new();
        output.push_str(ANSI_GRAY);
        for _ in 0..$self.indent {
            output.push_str("|  ");
        }
        output.push_str(ANSI_RESET);
        let val = &format!($fmt $(, $args)*);

        // Colorize everything before a colon
        let mut parts = val.splitn(2, ':');
        let first_part = parts.next().unwrap_or("");
        let second_part = parts.next();
        
        output.push_str(ANSI_BOLD);
        output.push_str(first_part);
        output.push_str(ANSI_RESET);

        if let Some(second_part) = second_part {
            output.push_str(":");
            output.push_str(second_part);
        }

        output
    }};
}

impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter { indent: 0 }
    }

    pub fn print_program(&mut self, program: &Program) -> String {
        self.indent = 0;
        let mut output = String::new();
        for declaration in &program.declarations {
            output.push_str(&self.print_declaration(declaration));
        }
        output
    }

    fn print_declaration(&mut self, declaration: &Declaration) -> String {
        match declaration {
            Declaration::Function { name, params, return_type, body, generic_args } => {
                let mut output = fmt_indent!(self, "Function: {}\n", name);
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Parameters:\n"));
                for param in params {
                    output.push_str(&fmt_indent!(self, "- {}: {}\n", param.name, self.print_type(&param.param_type)));
                }
                output.push_str(&fmt_indent!(self, "Return Type: {}\n", self.print_type(return_type)));
                output.push_str(&fmt_indent!(self, "Body: "));
                output.push_str(&self.print_expression(body));
                self.indent -= 1;
                output
            }
            Declaration::Import { path } => {
                fmt_indent!(self, "Import: {}\n", path.join("."))
            }
            Declaration::Struct { name, elements, generic_args } => {
                let mut output = fmt_indent!(self, "Struct: {}\n", name);
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Elements:\n"));
                for element in elements {
                    match element {
                        StructElement::Field { name, field_type } => {
                            output.push_str(&fmt_indent!(self, "- {}: {}\n", name, self.print_type(field_type)));
                        },
                        StructElement::Declaration(declaration) => {
                            output.push_str(&self.print_declaration(declaration));
                        }
                    }
                }
                self.indent -= 1;
                output
            },
            Declaration::TypeDeclaration { name, alias, generic_args } => {
                let mut output = fmt_indent!(self, "Type Declaration: {}\n", name);
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Alias: {}\n", self.print_type(alias)));
                if !generic_args.is_empty() {
                    output.push_str(&fmt_indent!(self, "Generic Arguments:\n"));
                    for arg in generic_args {
                        output.push_str(&fmt_indent!(self, "- {}\n", arg));
                    }
                }
                self.indent -= 1;
                output
            }
        }
    }

    fn print_expression(&mut self, expression: &Expression) -> String {
        match expression {
            Expression::Assignment { name: variable, value, .. } => {
                let mut output = fmt_indent!(self, "Assignment:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Variable: {}\n", variable));
                output.push_str(&fmt_indent!(self, "Value:\n"));
                output.push_str(&self.print_expression(value));
                self.indent -= 1;
                output
            },
            Expression::BinaryOperation { left, operator, right } => {
                let mut output = fmt_indent!(self, "Binary Operation: {}\n", operator);
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Left:\n"));
                output.push_str(&self.print_expression(left));
                output.push_str(&fmt_indent!(self, "Right:\n"));
                output.push_str(&self.print_expression(right));
                self.indent -= 1;
                output
            },
            Expression::UnaryOperation { operator, operand } => {
                let mut output = fmt_indent!(self, "Unary Operation: {}\n", operator);
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Operand:\n"));
                output.push_str(&self.print_expression(operand));
                self.indent -= 1;
                output
            },
            Expression::Block(statements) => {
                let mut output = fmt_indent!(self, "Block:\n");
                self.indent += 1;
                for statement in statements {
                    output.push_str(&self.print_statement(statement));
                }
                self.indent -= 1;
                output
            },
            Expression::BooleanLiteral(value) => {
                fmt_indent!(self, "Boolean Literal: {}\n", value)
            },
            Expression::CharLiteral(value) => {
                fmt_indent!(self, "Character Literal: {}\n", value)
            },
            Expression::NumberLiteral(value) => {
                fmt_indent!(self, "Number Literal: {}\n", value)
            },
            Expression::StringLiteral(value) => {
                fmt_indent!(self, "String Literal: {}\n", value)
            },
            Expression::FunctionCall { callee, args } => {
                let mut output = fmt_indent!(self, "Function Call\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Callee:\n"));
                output.push_str(&self.print_expression(callee));

                output.push_str(&fmt_indent!(self, "Arguments:\n"));
                for arg in args {
                    output.push_str(&self.print_expression(arg));
                }
                self.indent -= 1;
                output
            },
            Expression::Variable { name, .. } => {
                fmt_indent!(self, "Variable: {}\n", name)
            },
            Expression::If { condition, then_branch, else_branch } => {
                let mut output = fmt_indent!(self, "If Statement:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Condition:\n"));
                output.push_str(&self.print_expression(condition));
                output.push_str(&fmt_indent!(self, "Then Branch:\n"));
                output.push_str(&self.print_expression(then_branch));
                if let Some(else_branch) = else_branch {
                    output.push_str(&fmt_indent!(self, "Else Branch:\n"));
                    output.push_str(&self.print_expression(else_branch));
                }
                self.indent -= 1;
                output
            },
            Expression::Loop(LoopType::Infinite { body }) => {
                let mut output = fmt_indent!(self, "Infinite Loop:\n");
                self.indent += 1;
                output.push_str(&self.print_expression(body));
                self.indent -= 1;
                output
            },
            Expression::Loop(LoopType::While { condition, body }) => {
                let mut output = fmt_indent!(self, "While Loop:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Condition:\n"));
                output.push_str(&self.print_expression(condition));
                output.push_str(&fmt_indent!(self, "Body: "));
                output.push_str(&self.print_expression(body));
                self.indent -= 1;
                output
            },
            Expression::Loop(LoopType::Iterator { mutability, iterator, iterable, body }) => {
                let mut output = fmt_indent!(self, "Iterator Loop:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Mutability: {}\n", match mutability {
                    VariableMutability::Mutable => "Mutable",
                    VariableMutability::Immutable => "Immutable",
                }));
                output.push_str(&fmt_indent!(self, "Iterator: {}\n", iterator));
                output.push_str(&fmt_indent!(self, "Iterable:\n"));
                output.push_str(&self.print_expression(iterable));
                output.push_str(&fmt_indent!(self, "Body: "));
                output.push_str(&self.print_expression(body));
                self.indent -= 1;
                output
            },
            Expression::MemberAccess { object, member } => {
                let mut output = fmt_indent!(self, "Member Access:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Object:\n"));
                output.push_str(&self.print_expression(object));
                output.push_str(&fmt_indent!(self, "Member: {}\n", member));
                self.indent -= 1;
                output
            }
            Expression::Array { array_type, size, initial_value } => {
                let mut output = fmt_indent!(self, "Array:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Type: {}\n", self.print_type(array_type)));
                output.push_str(&fmt_indent!(self, "Size:\n"));
                output.push_str(&self.print_expression(size));
                output.push_str(&fmt_indent!(self, "Initial Value:\n"));
                output.push_str(&self.print_expression(initial_value));
                self.indent -= 1;
                output
            },
            Expression::StructCreation { struct_type, fields } => {
                let mut output = fmt_indent!(self, "Struct Creation:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Type: {}\n", self.print_type(struct_type)));
                output.push_str(&fmt_indent!(self, "Fields:\n"));
                for (name, value) in fields {
                    output.push_str(&fmt_indent!(self, "{}:\n", name));
                    self.indent += 1;
                    output.push_str(&self.print_expression(value));
                    self.indent -= 1;
                }
                self.indent -= 1;
                output
            }
        }
    }

    fn print_statement(&mut self, statement: &Statement) -> String {
        match statement {
            Statement::Declaration(declaration) => {
                self.print_declaration(declaration)
            },
            Statement::Break => {
                fmt_indent!(self, "Break\n")
            },
            Statement::Continue => {
                fmt_indent!(self, "Continue\n")
            },
            Statement::Expression { expression, result } => {
                let mut output = fmt_indent!(self, "Expression:\n");
                self.indent += 1;
                output.push_str(&self.print_expression(expression));
                if *result {
                    output.push_str(&fmt_indent!(self, "Result: true\n"));
                }
                self.indent -= 1;
                output
            },
            Statement::Return(value) => {
                let mut output = fmt_indent!(self, "Return:\n");
                self.indent += 1;
                if let Some(value) = value {
                    output.push_str(&self.print_expression(value));
                } else {
                    output.push_str(&fmt_indent!(self, "No value\n"));
                }
                self.indent -= 1;
                output
            },
            Statement::VariableDeclaration { mutability, name, variable_type, value } => {
                let mut output = fmt_indent!(self, "Variable Declaration: {}\n", name);
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Mutability: {}\n", match mutability {
                    VariableMutability::Mutable => "Mutable",
                    VariableMutability::Immutable => "Immutable",
                }));
                output.push_str(&fmt_indent!(self, "Type: {}\n", self.print_type(variable_type)));
                output.push_str(&fmt_indent!(self, "Value:\n"));
                output.push_str(&self.print_expression(value));
                self.indent -= 1;
                output
            }
        }
    }

    fn print_type(&mut self, ty: &Type) -> String {
        match ty {
            Type::Boolean => "Boolean".to_string(),
            Type::Character => "Character".to_string(),
            Type::F32 => "F32".to_string(),
            Type::F64 => "F64".to_string(),
            Type::I8 => "I8".to_string(),
            Type::I16 => "I16".to_string(),
            Type::I32 => "I32".to_string(),
            Type::I64 => "I64".to_string(),
            Type::U8 => "U8".to_string(),
            Type::U16 => "U16".to_string(),
            Type::U32 => "U32".to_string(),
            Type::U64 => "U64".to_string(),
            Type::Nil => "Nil".to_string(),
            Type::Identifier { name, generics: generic_args } => {
                let mut output = name.clone();
                if !generic_args.is_empty() {
                    output.push('<');
                    output.push_str(&generic_args.iter().map(|arg| self.print_type(arg)).collect::<Vec<_>>().join(", "));
                    output.push('>');
                }
                output
            },
            Type::Array(of) => {
                let mut output = "\n".to_string();
                output.push_str(&fmt_indent!(self, "Array of "));
                self.indent += 1;
                output.push_str(&self.print_type(of));
                self.indent -= 1;
                output
            }
            Type::Function { params, return_type } => {
                let mut output = fmt_indent!(self, "Function:\n");
                self.indent += 1;
                output.push_str(&fmt_indent!(self, "Parameters:\n"));
                for param in params {
                    output.push_str(&fmt_indent!(self, "- {}\n", self.print_type(&param)));
                }
                output.push_str(&fmt_indent!(self, "Return Type: {}\n", self.print_type(return_type)));
                self.indent -= 1;
                output
            }
        }
    }
}