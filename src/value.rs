use std::ops::Index;

pub type Value = f64;

pub struct ConstantPool {
    constants: Vec<Value>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
        }
    }

    pub fn push_value(&mut self, value: Value) -> usize {
        let index = self.constants.len();
        self.constants.push(value);
        index
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }
}

impl Default for ConstantPool {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for ConstantPool {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.constants[index]
    }
}
