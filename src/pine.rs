use crate::error::PineError;
use crate::Vector;

use std::fs::{create_dir_all, read_dir, remove_file, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use bincode::{deserialize, serialize};

#[derive(Debug)]
pub struct Pine {
    pub path: PathBuf,
    pub chunk_similarity: f32,
}

impl Pine {
    /// Create a new Pine instance with the given path and chunk_similarity threshold.
    pub fn new(path: PathBuf, chunk_similarity: f32) -> Result<Self, PineError> {
        create_dir_all(&path.join("vectors"))?;
        create_dir_all(&path.join("index"))?;

        Ok(Self { path, chunk_similarity })
    }

    /// Get the path to a vector's data file using its ID.
    fn get_index(&self, id: &str) -> Result<Option<PathBuf>, PineError> {
        let index_path = self.path.join("index").join(id);
        if index_path.exists() {
            let mut file = File::open(index_path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let path = PathBuf::from(String::from_utf8(buffer)?);
            Ok(Some(path.join(id)))
        } else {
            Ok(None)
        }
    }

    /// Set the path to a vector's data file using its ID.
    fn set_index(&self, id: &str, path: &PathBuf) -> Result<(), PineError> {
        let mut file = BufWriter::new(File::create(self.path.join("index").join(id))?);
        file.write_all(path.to_str().ok_or(PineError::PathConversion)?.as_bytes())?;
        Ok(())
    }

    /// Find the directory containing vectors with similarity above the threshold.
    fn find_dir(&self, vector: &Vector) -> Result<Option<PathBuf>, PineError> {
        for entry in read_dir(self.path.join("vectors"))? {
            let path = entry?.path();

            if path.is_dir() {
                let metadata_path = path.join("metadata");
                if metadata_path.exists() {
                    let mut file = BufReader::new(File::open(metadata_path)?);
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    let metadata: Vector = deserialize(&buffer)?;

                    if self.cosine_similarity(&vector, &metadata)? > self.chunk_similarity {
                        return Ok(Some(path));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Save a vector to the storage.
    pub fn save(&self, vector: &Vector) -> Result<(), PineError> {
        let serialized = serialize(&vector)?;

        let path = match self.find_dir(&vector)? {
            Some(path) => path,
            None => {
                let current_size = read_dir(&self.path.join("vectors"))?.count();
                let path = self.path.join("vectors").join(format!("{:03}", current_size));
                create_dir_all(&path)?;

                let mut file = BufWriter::new(File::create(path.join("metadata"))?);
                file.write_all(&serialized)?;

                path
            }
        };

        self.delete(&vector.id)?;

        let mut file = BufWriter::new(File::create(path.join(&vector.id))?);
        file.write_all(&serialized)?;

        self.set_index(&vector.id, &path)?;

        Ok(())
    }

    /// Load a vector from the storage by ID.
    pub fn load(&self, id: &str) -> Result<Option<Vector>, PineError> {
        if let Some(path) = self.get_index(&id)? {
            let mut file = BufReader::new(File::open(path)?);
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let vector: Vector = deserialize(&buffer)?;
            Ok(Some(vector))
        } else {
            Ok(None)
        }
    }

        /// Delete a vector from the storage by ID.
    pub fn delete(&self, id: &str) -> Result<(), PineError> {
        if let Some(path) = self.get_index(&id)? {
            remove_file(path)?;
            let index_path = self.path.join("index").join(id);
            if index_path.exists() {
                remove_file(index_path)?;
            }
        }
        Ok(())
    }

    /// Check if a vector with the given ID exists in the storage.
    pub fn exists(&self, id: &str) -> Result<bool, PineError> {
        Ok(self.get_index(&id)?.is_some())
    }

    /// Get the number of vectors in the storage.
    pub fn size(&self) -> Result<usize, PineError> {
        let mut size = 0;
        for entry in read_dir(self.path.join("vectors"))? {
            let path = entry?.path();

            if path.is_dir() {
                for entry in read_dir(path)? {
                    let path = entry?.path();
                    if path.is_file() && !path.ends_with("metadata") {
                        size += 1;
                    }
                }
            }
        }
        Ok(size)
    }

    /// Calculate the Euclidean distance between two vectors.
    pub fn distance(&self, query_vector: &Vector, stored_vector: &Vector) -> f32 {
        let diff = &query_vector.data - &stored_vector.data;
        let squared_diff = diff.mapv(|x| x * x);
        squared_diff.sum().sqrt()
    }

    /// Calculate the cosine similarity between two vectors.
    pub fn cosine_similarity(&self, query_vector: &Vector, stored_vector: &Vector) -> Result<f32, PineError> {
        let dot_product = query_vector.data.dot(&stored_vector.data);
        let query_norm = query_vector.data.dot(&query_vector.data).sqrt();
        let stored_norm = stored_vector.data.dot(&stored_vector.data).sqrt();
        if (query_norm * stored_norm) != 0.0 {
            Ok(dot_product / (query_norm * stored_norm))
        } else {
            Ok(0.0)
        }
    }
}
