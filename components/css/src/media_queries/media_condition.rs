use super::media_feature_expression::MediaFeatureExpression;

/// A binary `and` or `or` operator.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[allow(missing_docs)]
pub enum Operator {
    And,
    Or,
}

/// Represents a media condition.
#[derive(Clone, Debug, PartialEq)]
pub enum MediaCondition {
    /// A simple media feature expression, implicitly parenthesized.
    Feature(MediaFeatureExpression),
    /// A negation of a condition.
    Not(Box<MediaCondition>),
    /// A set of joint operations.
    Operation(Box<[MediaCondition]>, Operator),
    /// A condition wrapped in parenthesis.
    InParens(Box<MediaCondition>),
}
