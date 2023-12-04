pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self, input: &str) -> usize;
    fn part_b(&self, input: &str) -> usize;
}
