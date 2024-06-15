use ndarray::Array1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vector {
    pub id: String,
    pub data: Array1<f32>,
}

impl Vector {
    pub fn new(id: String, data: Array1<f32>) -> Self {
        Vector { id, data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
