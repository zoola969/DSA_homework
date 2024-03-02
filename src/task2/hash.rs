#[derive(Debug)]
pub struct CustomSetExceedsCapacityError;

pub struct CustomHasSet {
    data: Vec<Option<i32>>,
    size: usize,
}

impl CustomHasSet {
    pub fn new(size: usize) -> Self {
        Self {
            data: (0..size).map(|_| None).collect(),
            size,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn add(&mut self, value: i32) -> Result<bool, CustomSetExceedsCapacityError> {
        if self.size == 0 {
            return Err(CustomSetExceedsCapacityError);
        }
        let orig_hash = self.hash(value);
        let mut next_hash = orig_hash;
        loop {
            next_hash = self.rehash(next_hash);
            if next_hash >= self.size {
                next_hash = 0;
            }
            if next_hash == orig_hash {
                return Err(CustomSetExceedsCapacityError);
            }
            if let Some(val) = self.data[next_hash] {
                if val == value {
                    return Ok(false);
                }
                continue;
            }
            self.data[next_hash] = Some(value);
            return Ok(true);
        }
    }

    pub fn contains(&self, value: i32) -> bool {
        if self.size == 0 {
            return false;
        }
        let orig_hash = self.hash(value);
        let mut next_hash = orig_hash;
        loop {
            next_hash = self.rehash(next_hash);
            if next_hash >= self.size {
                next_hash = 0;
            }
            if next_hash == orig_hash {
                return false;
            }
            if let Some(val) = self.data[next_hash] {
                if val == value {
                    return true;
                }
                continue;
            }
        }
    }

    fn hash(&self, value: i32) -> usize {
        value.unsigned_abs() as usize % self.size
    }
    fn rehash(&self, prev_hash: usize) -> usize {
        prev_hash + 1
    }
}
