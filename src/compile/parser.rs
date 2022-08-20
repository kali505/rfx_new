#[allow(unused_imports)]
use log::{debug, error, info, warn};

struct VariableType {
    name: String,
    size: usize, // into byte
}

struct VariableName<'a> {
    name: String,
    tp: &'a VariableType,
}

struct FunctionName<'a> {
    name: String,
    args: Vec<VariableName<'a>>,
    return_tp: &'a VariableType,
}

#[allow(dead_code)]
enum KeywordType {
    None,
    VariableType,
    VariableName,

    FunctionName,
}

fn init() -> (
    Vec<VariableType>,
    Vec<VariableName<'static>>,
    Vec<FunctionName<'static>>,
) {
    let mut variable_type: Vec<VariableType> = Vec::new();
    let mut variable_name: Vec<VariableName> = Vec::new();
    let mut function_name: Vec<FunctionName> = Vec::new();

    variable_type = vec![
        VariableType {
            name: "i8".to_string(),
            size: 1,
        },
        VariableType {
            name: "i16".to_string(),
            size: 2,
        },
        VariableType {
            name: "i32".to_string(),
            size: 4,
        },
        VariableType {
            name: "i64".to_string(),
            size: 8,
        },
        VariableType {
            name: "u8".to_string(),
            size: 1,
        },
        VariableType {
            name: "u16".to_string(),
            size: 2,
        },
        VariableType {
            name: "u32".to_string(),
            size: 4,
        },
        VariableType {
            name: "u64".to_string(),
            size: 8,
        },
        VariableType {
            name: "bool".to_string(),
            size: 1,
        },
        VariableType {
            name: "char".to_string(),
            size: 4,
        },
    ];

    return (variable_type, variable_name, function_name);
}

trait ParseCheck {
    fn is_variable_type(self, v: Vec<VariableType>) -> Option<usize>;
    fn is_variable_name(self, v: Vec<VariableName>) -> Option<usize>;
    fn is_function_name(self, v: Vec<FunctionName>) -> Option<usize>;
}

impl ParseCheck for String {
    fn is_variable_type(self, v: Vec<VariableType>) -> Option<usize> {
        return v.iter().position(|x| x.name == self);
    }

    fn is_variable_name(self, v: Vec<VariableName>) -> Option<usize> {
        return v.iter().position(|x| x.name == self);
    }

    fn is_function_name(self, v: Vec<FunctionName>) -> Option<usize> {
        return v.iter().position(|x| x.name == self);
    }
}
