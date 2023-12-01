use super::Answer;

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self, input: &str) -> Answer;
    fn part_b(&self, input: &str) -> Answer;
}
