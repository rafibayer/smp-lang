use super::*;

// evaluates the truthiness of a f64 value
pub fn truthy(value: f64) -> bool {
    value.abs() > EPSILON
}

// converts a boolean to a float
pub fn bool_to_float(bool: bool) -> f64 {
    (bool as u32) as f64
}

// Attempts to get the value of an expression that may not return a value.
// if no value can be unwrapped, returns a ValuelessExpression interpreter error
pub fn get_expression_result_value(
    exp: &Exp,
    res: Result<Option<Value>, InterpreterError>,
) -> Result<Value, InterpreterError> {
    match res? {
        Some(value) => Ok(value),
        None => Err(InterpreterError::ValuelessExpression(exp.clone())),
    }
}
