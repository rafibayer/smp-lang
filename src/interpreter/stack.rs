// use super::symbols::*;
// use super::InterpreterError;
// use super::Value;

// pub struct Stack {
//     stack: Vec<Element>
// }


// enum Element {
//     Op(OpKind),
//     Value(Value),
// }

// impl Stack {
//     pub fn new() -> Stack {
//         Stack {
//             stack: Vec::new(),
//         }
//     }

//     pub fn push_infix(&mut self, lhs: Value, op: OpKind, rhs: Value) -> Result<Value, InterpreterError> {
//         self.stack.push(Element::Value(lhs));
//         self.stack.push(Element::Op(op));
//         self.stack.push(Element::Value(rhs));


//     }



// }

// /// returns the operator precedence of a given expkind.
// /// if expkind is not an operation, returns PrecedenceError
// fn get_precedence(operation: &ExpKind) -> Result<u32, InterpreterError> {
//     match operation {
//         // calls and subscripts
//         ExpKind::ArrayAccess { name: _, index: _ } => Ok(1),
//         ExpKind::Call(_, _) => Ok(1),
//         ExpKind::BuiltIn(_) => Ok(1),

//         // unary operators
//         ExpKind::Unary(_, _) => Ok(2),
//         // mul div, mod
//         ExpKind::Infix(_, Op{op: OpKind::Mul}, _) => Ok(3),
//         ExpKind::Infix(_, Op{op: OpKind::Div}, _) => Ok(3),
//         ExpKind::Infix(_, Op{op: OpKind::Mod}, _) => Ok(3),

//         // plus, minus
//         ExpKind::Infix(_, Op{op: OpKind::Plus}, _) => Ok(4),
//         ExpKind::Infix(_, Op{op: OpKind::Minus}, _) => Ok(4),

//         // comparison
//         ExpKind::Infix(_, Op{op: OpKind::Comparison(Comparison { comparison: ComparisonKind::Less })}, _) => Ok(5),
//         ExpKind::Infix(_, Op{op: OpKind::Comparison(Comparison { comparison: ComparisonKind::LessEqual })}, _) => Ok(5),
//         ExpKind::Infix(_, Op{op: OpKind::Comparison(Comparison { comparison: ComparisonKind::More })}, _) => Ok(5),
//         ExpKind::Infix(_, Op{op: OpKind::Comparison(Comparison { comparison: ComparisonKind::MoreEqual })}, _) => Ok(5),

//         // equality
//         ExpKind::Infix(_, Op{op: OpKind::Comparison(Comparison { comparison: ComparisonKind::Equals })}, _) => Ok(6),
//         ExpKind::Infix(_, Op{op: OpKind::Comparison(Comparison { comparison: ComparisonKind::NotEqual })}, _) => Ok(6),

//         // logical AND
//         ExpKind::Infix(_, Op{op: OpKind::Logical(Logical { logical: LogicalKind::And })}, _) => Ok(7),
        
//         // logical OR
//         ExpKind::Infix(_, Op{op: OpKind::Logical(Logical { logical: LogicalKind::Or })}, _) => Ok(8),

//         other => return Err(InterpreterError::PrecedenceError(other.clone()))
//     }
// }