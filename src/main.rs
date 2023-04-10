use std::fs::{File, create_dir_all};
use std::io::{Write, Read};
use std::path::PathBuf;
use bincode::{serialize, deserialize};

use ndarray::Array1;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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
    path:PathBuf,
}

impl Pine {
    
    fn new(path:PathBuf) -> Pine {
        create_dir_all(&path).expect("failed to create directory");
        Pine { path }
    }
    
    fn save(&self, vector:&Vector) {
        let path = self.path.join(&vector.id);
        let serialized = serialize(&vector).expect("failed to serialize");
        let mut file = File::create(path).expect("failed to create file");
        file.write_all(&serialized).expect("failed to write file");
    }
    
    fn load(&self, id:&String) -> Option<Vector> {
        let path = self.path.join(&id);
        if !path.exists() { None } else {
            let mut file = File::open(path).expect("failed to open file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("failed to read file");
            let vector:Vector = deserialize(&buffer).expect("failed to deserialize");
            Some(vector)
        }
    }

    fn remove(&self, id:&String) {
        let path = self.path.join(&id);
        if path.exists() {
            std::fs::remove_file(path).expect("failed to remove file");
        }
    }

    fn exists(&self, id:&String) -> bool {
        let path = self.path.join(&id);
        path.exists()
    }

    fn size(&self) -> usize {
        std::fs::read_dir(&self.path).expect("failed to read directory").count()
    }

    fn clear(&self) {
        for entry in std::fs::read_dir(&self.path).expect("failed to read directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                std::fs::remove_file(path).expect("failed to remove file");
            }
        }
    }

    fn distance(query_vector:&Vector, stored_vector:&Vector) -> f32 {
        let diff = &query_vector.data - &stored_vector.data;
        let squared_diff = diff.mapv(|x| x * x);
        squared_diff.sum().sqrt() 
    }

    fn cosine_similarity(query_vector:&Vector, stored_vector:&Vector) -> f32 {
        let dot_product = query_vector.data.dot(&stored_vector.data);
        let query_norm = query_vector.data.dot(&query_vector.data).sqrt();
        let stored_norm = stored_vector.data.dot(&stored_vector.data).sqrt();
        dot_product / (query_norm * stored_norm)
    }
    
    fn query_nearest_neighbor(&self, query_vector:&Vector) -> Option<Vector> {
        let mut max_similarity:f32 = f32::MIN;
        let mut nearest_vector:Option<Vector> = None;

        for entry in std::fs::read_dir(&self.path).expect("failed to read directory") {
            if let Ok(entry) = entry {
               let id = entry.file_name().into_string().expect("failed to convert file name");

               if let Some(stored_vector) = self.load(&id) {
                   let similarity = Pine::cosine_similarity(query_vector, &stored_vector);
                   if similarity > max_similarity {
                       max_similarity = similarity;
                       nearest_vector = Some(stored_vector);
                   }
               }
            }
        }

        nearest_vector
    }

}

fn main() {
    let pine = Pine::new(PathBuf::from("./vectors"));

    let data = Array1::from(vec![1.0, 2.0, 3.0]);
    let vector = Vector { id: "test".to_string(), data };

    println!("pine: {:?}", pine);
    println!("pine.path: {:?}", pine.path);

    println!("vector: {:?}", vector);
    println!("vector.id: {:?}", vector.id);
    println!("vector.data: {:?}", vector.data);
    println!("vector.size: {:?}", vector.size());

    pine.save(&vector);

    let load_vector = pine.load(&vector.id);
    println!("load_vector:{:?}", load_vector);
   
    let a = Vector { id: "a".to_string(), data: Array1::from(vec![1.0, 2.0, 3.0]) };
    let b = Vector { id: "b".to_string(), data: Array1::from(vec![2.0, 3.0, 4.0]) };
    let c = Vector { id: "c".to_string(), data: Array1::from(vec![3.0, 4.0, 5.0]) };

    pine.save(&a);
    pine.save(&b);
    pine.save(&c);

    let exists = pine.exists(&a.id);
    println!("exists: {:?}", exists);

    println!("size: {:?}", pine.size());

    let distance = Pine::distance(&a, &b);
    println!("distance: {:?}", distance);

    let z = Vector { id: "z".to_string(), data: Array1::from(vec![5.0, 3.0, 4.0]) };
    println!("z: {:?}", z);

    let neighbor = pine.query_nearest_neighbor(&z);
    println!("query_nearest_neighbor: {:?}", neighbor);

    pine.remove(&a.id);
    pine.remove(&b.id);
    pine.remove(&c.id);

    pine.clear();
}
