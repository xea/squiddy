use super::matcher::Matcher;
use super::action::Action;

pub struct Condition {
    pub matcher: Box<Matcher>,
    pub actions: Vec<Box<Action>>
}
