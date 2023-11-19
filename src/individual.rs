#[derive(Clone, Debug)]
pub struct Individual<T> {
    pub chromosome: Vec<bool>,
    pub fitness_score: T,
}

impl<T> PartialEq for Individual<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.fitness_score == other.fitness_score
    }
}

impl<T> Eq for Individual<T> where T: Eq {}

impl<T> PartialOrd for Individual<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness_score.partial_cmp(&other.fitness_score)
    }
}

impl<T> Ord for Individual<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness_score.cmp(&other.fitness_score)
    }
}

impl<T: Default> Default for Individual<T> {
    fn default() -> Individual<T> {
        Individual {
            chromosome: Vec::new(),
            fitness_score: T::default(),
        }
    }
}
