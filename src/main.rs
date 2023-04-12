use std::fs::{File, create_dir_all, read_dir, remove_file};
use std::io::{Write, Read};
use std::path::PathBuf;
use bincode::{serialize, deserialize};
use ndarray::Array1;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Vector {
    id: String,
    data: Array1<f32>,
}

impl Vector {
    fn size(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug)]
struct Pine {
    path: PathBuf,
    chunk_similarity: f32,
}

impl Pine {
    fn new(path: PathBuf, chunk_similarity: f32) -> Pine {
        create_dir_all(&path.join("vectors")).expect("failed to create directory");
        create_dir_all(&path.join("index")).expect("failed to create directory");
        
        return Pine { path, chunk_similarity };
    }

    fn get_index(&self, id:&String) -> Option<PathBuf> {
        let index_path = self.path.join("index").join(id);
        if index_path.exists() {
            let mut file = File::open(index_path).expect("failed to open file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("failed to read file");
            let path = PathBuf::from(String::from_utf8(buffer).expect("failed to convert to string"));
            return Some(path.join(id));
        }
        return None;
    }

    fn set_index(&self, id:&String, path:&PathBuf) {
        let mut file = File::create(self.path.join("index").join(id)).expect("failed to create file");
        file.write_all(path.to_str().unwrap().as_bytes()).expect("failed to write file");
    }

    fn find_dir(&self, vector:&Vector) -> Option<PathBuf> {
        for entry in read_dir(self.path.join("vectors")).expect("failed to read directory") {
            let path = entry.expect("failed to read directory").path();
            
            if path.is_dir() {
                let metadata_path = path.join("metadata"); 
                if metadata_path.exists() {
                    let mut file = File::open(metadata_path).expect("failed to open file");
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).expect("failed to read file");
                    let metadata: Vector = deserialize(&buffer).expect("failed to deserialize vector");

                    if self.cosine_similarity(&vector, &metadata) > self.chunk_similarity {
                        return Some(path); 
                    }
                }
            }
        }
        return None;
    }

    fn save(&self, vector:&Vector) {
        let serialized = serialize(&vector).expect("failed to serialize vector");
        
        let path = self.find_dir(&vector).unwrap_or_else(|| {
            let current_size = read_dir(&self.path.join("vectors")).expect("failed to read directory").count();
            let path = self.path.join("vectors").join(format!("{:03}", current_size));
            create_dir_all(&path).expect("failed to create directory");

            let mut file = File::create(path.join("metadata")).expect("failed to create file");
            file.write_all(&serialized).expect("failed to write file");
            
            return path;
        });

        self.delete(&vector.id);

        let mut file = File::create(path.join(&vector.id)).expect("failed to create file");
        file.write_all(&serialized).expect("failed to write file");

        self.set_index(&vector.id, &path);
    }

    fn load(&self, id:&String) -> Option<Vector> {
        let path = self.get_index(&id);
        if path.is_some() {
            let mut file = File::open(path.unwrap()).expect("failed to open file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("failed to read file");
            let vector: Vector = deserialize(&buffer).expect("failed to deserialize vector");
            return Some(vector);
        }
        return None;
    }

    fn delete(&self, id:&String) {
        let path = self.get_index(&id);
        if path.is_some() {
            remove_file(path.unwrap()).expect("failed to delete file");
        }
    }

    fn exists(&self, id:&String) -> bool {
        return self.get_index(&id).is_some();
    }

    fn size(&self) -> usize {
        let mut size = 0;
        for entry in read_dir(self.path.join("vectors")).expect("failed to read directory") {
            let path = entry.expect("failed to read directory").path();
            
            if path.is_dir() {
                for entry in read_dir(path).expect("failed to read directory") {
                    let path = entry.expect("failed to read directory").path();
                    if path.is_file() {
                        size += 1;
                    }
                }
            }
        }
        return size;
    }

    fn distance(&self, query_vector:&Vector, stored_vector:&Vector) -> f32 {
        let diff = &query_vector.data - &stored_vector.data;
        let squared_diff = diff.mapv(|x| x * x);
        return squared_diff.sum().sqrt();
    }

    fn cosine_similarity(&self, query_vector:&Vector, stored_vector:&Vector) -> f32 {
        let dot_product = query_vector.data.dot(&stored_vector.data);
        let query_norm = query_vector.data.dot(&query_vector.data).sqrt();
        let stored_norm = stored_vector.data.dot(&stored_vector.data).sqrt();
        return dot_product / (query_norm * stored_norm);
    }
}

fn main() {
    let pine = Pine::new(PathBuf::from("./pine"), 0.99);

    println!("pine: {:?}", pine);

    let abc = Vector { id: "abc".to_string(), data: Array1::from(vec![1.0, 2.0, -3.0, 4.0, -5.0]) };
    pine.save(&abc);

    println!("vector_size: {}", &abc.size());
    
    let xyx = Vector { id: "xyz".to_string(), data: Array1::from(vec![1.0, 2.0, -3.0, 4.0, -5.0]) };
    pine.save(&xyx);
    
    for i in 0..20 {
        let id = format!("{}", i);
        let data = vec![i as f32 + 1.0, i as f32 + 2.0, i as f32 + 3.0, i as f32 + 4.0, i as f32 + 5.0];
        pine.save(&Vector { id, data: Array1::from(data) });
    }
    
    println!("size: {}", pine.size());
    println!("exists: {}", pine.exists(&"abc".to_string()));

    println!("distance: {}", pine.distance(&abc, &xyx));

    let ghi = pine.load(&"abc".to_string()).unwrap();
    println!("def: {:?}", ghi);
}
