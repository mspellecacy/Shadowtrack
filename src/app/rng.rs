use rand::Rng;

/// Trait for abstracting over random number generation.
pub trait RandomSource {
    fn roll_range(&mut self, min: u32, max: u32) -> u32;
    fn choose<'a, T>(&mut self, list: &'a [T]) -> Option<&'a T>;
}

/// Default implementation using rng()
pub struct DefaultRandomSource;

impl RandomSource for DefaultRandomSource {
    fn roll_range(&mut self, min: u32, max: u32) -> u32 {
        rand::rng().random_range(min..=max)
    }

    fn choose<'a, T>(&mut self, list: &'a [T]) -> Option<&'a T> {
        if list.is_empty() {
            None
        } else {
            let index = self.roll_range(0, (list.len() - 1) as u32) as usize;
            list.get(index)
        }
    }
}
