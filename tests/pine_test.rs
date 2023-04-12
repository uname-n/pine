// use std::fs;
// use std::path::PathBuf;

use ndarray::Array1;
use tempdir::TempDir;

use pine::Pine;
use pine::Vector;

fn create_temp_pine(chunk_similarity: f32) -> Pine {
    let temp_dir = TempDir::new("pine_test").expect("Cannot create temp directory");
    let temp_path = temp_dir.into_path();
    Pine::new(temp_path, chunk_similarity).expect("Cannot create Pine instance")
}

#[test]
fn test_save_load_exists() {
    let pine = create_temp_pine(0.9);
    let vector = Vector::new("1".to_string(), Array1::from_vec(vec![0.5, 0.3, 0.7]));
    pine.save(&vector).expect("Save failed");

    assert!(pine.exists(&"1".to_string()).expect("Exists failed"));
    let loaded_vector = pine.load(&"1".to_string()).expect("Load failed").expect("Vector not found");
    assert_eq!(vector.id, loaded_vector.id);
    assert_eq!(vector.data, loaded_vector.data);
}

#[test]
fn test_save_new_directory() {
    let pine = create_temp_pine(0.5);
    let vector1 = Vector::new("1".to_string(), Array1::from_vec(vec![0.5, 0.3, 0.7]));
    let vector2 = Vector::new("2".to_string(), Array1::from_vec(vec![0.2, 0.4, 0.1]));

    pine.save(&vector1).expect("Save failed");
    pine.save(&vector2).expect("Save failed");

    let loaded_vector = pine.load(&"2".to_string()).expect("Load failed").expect("Vector not found");
    assert_eq!(vector2.id, loaded_vector.id);
    assert_eq!(vector2.data, loaded_vector.data);
}

#[test]
fn test_save_existing_directory() {
    let pine = create_temp_pine(0.9);
    let vector1 = Vector::new("1".to_string(), Array1::from_vec(vec![0.5, 0.3, 0.7]));
    let vector2 = Vector::new("2".to_string(), Array1::from_vec(vec![0.55, 0.35, 0.75]));

    pine.save(&vector1).expect("Save failed");
    pine.save(&vector2).expect("Save failed");

    let loaded_vector = pine.load(&"2".to_string()).expect("Load failed").expect("Vector not found");
    assert_eq!(vector2.id, loaded_vector.id);
    assert_eq!(vector2.data, loaded_vector.data);
}

#[test]
fn test_delete() {
    let pine = create_temp_pine(0.9);
    let vector = Vector::new("1".to_string(), Array1::from_vec(vec![0.5, 0.3, 0.7]));
    pine.save(&vector).expect("Save failed");

    pine.delete(&"1".to_string()).expect("Delete failed");
    assert!(!pine.exists(&"1".to_string()).expect("Exists failed"));
    assert!(pine.load(&"1".to_string()).expect("Load failed").is_none());
}


/*
#[test]
fn test_size() {
    let pine = create_temp_pine(0.9);
    let vector1 = Vector::new("1".to_string(), Array1::from_vec(vec![0.5, 0.3, 0.7]));
    let vector2 = Vector::new("2".to_string(), Array1::from_vec(vec![0.2, 0.4, 0.1]));

    pine.save(&vector1).expect("Save failed");
    assert_eq!(1, pine.size().expect("Size failed"));

    pine.save(&vector2).expect("Save failed");
    assert_eq!(2, pine.size().expect("Size failed"));

    pine.delete(&"1".to_string()).expect("Delete failed");
    assert_eq!(1, pine.size().expect("Size failed"));
}

#[test]
fn test_distance() {
    let pine = create_temp_pine(0.9);
    let vector1 = Vector::new("1".to_string(), Array1::from_vec(vec![0.5, 0.3, 0.7]));
    let vector2 = Vector::new("2".to_string(), Array1::from_vec(vec![0.2, 0.4, 0.1]));

    let distance = pine.distance(&vector1, &vector2);
    assert!((0.5830952 - distance).abs() < 1e-6, "Incorrect distance: {}", distance);
}

#[test]
fn test_cosine_similarity() {
    let pine = create_temp_pine(0.9);
    let vector1 = Vector::new("1".to_string(), Array1::from_vec(vec![0.5, 0.3, 0.7]));
    let vector2 = Vector::new("2".to_string(), Array1::from_vec(vec![0.2, 0.4, 0.1]));

    let similarity = pine.cosine_similarity(&vector1, &vector2).expect("Cosine similarity calculation failed");
    assert!((0.7127598 - similarity).abs() < 1e-6, "Incorrect similarity: {}", similarity);
}


#[test]
fn test_empty_storage() {
    let pine = create_temp_pine(0.9);
    assert_eq!(0, pine.size().expect("Size failed"));
    assert!(!pine.exists(&"1".to_string()).expect("Exists failed"));
    assert!(pine.load(&"1".to_string()).expect("Load failed").is_none());
}
*/
