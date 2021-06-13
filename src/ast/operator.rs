
use crate::interpreter::symbols::Comparison;
use crate::interpreter::symbols::ComparisonKind;
use crate::interpreter::symbols::Logical;
use crate::tokens::Token;
use crate::interpreter::symbols::OpKind;
use crate::interpreter::symbols::LogicalKind;

use super::ASTError;


// lookup table to convert from token to corresponding infix operator,
// returns InvalidOperator error if token doesn't correspond to an OpKind
pub fn lookup_infix(token: Token) -> Result<OpKind, ASTError> {
    Ok(match token {
        // standard
        Token::Plus => OpKind::Plus,
        Token::Minus => OpKind::Minus,
        Token::Mul => OpKind::Mul,
        Token::Div => OpKind::Div,
        Token::Mod => OpKind::Mod,
        // logical
        Token::Or => OpKind::Logical(Logical{logical: LogicalKind::Or}),
        Token::And => OpKind::Logical(Logical{logical: LogicalKind::And}),
        // comparison
        Token::Equals => OpKind::Comparison(Comparison{comparison: ComparisonKind::Equals}),
        Token::Less => OpKind::Comparison(Comparison{comparison: ComparisonKind::Less}),
        Token::More => OpKind::Comparison(Comparison{comparison: ComparisonKind::More}),
        Token::LessEqual => OpKind::Comparison(Comparison{comparison: ComparisonKind::LessEqual}),
        Token::MoreEqual => OpKind::Comparison(Comparison{comparison: ComparisonKind::MoreEqual}),
        Token::NotEqual => OpKind::Comparison(Comparison{comparison: ComparisonKind::NotEqual}),

        other=> return Err(ASTError::InvalidOperator(other))
    })
}
