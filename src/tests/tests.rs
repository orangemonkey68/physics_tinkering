use crate::structs::math::{Vector, Dot, Scalar, Matrix, Det};

#[test]
fn test_vectors() {
    test_dot_product();
    test_vector_add();
    test_vector_scalar();
    test_vector_subtraction();
}

fn test_matrices() {
    test_matrix_det();
    test_matrix_validity();
    test_matrix_square();
}

fn test_matrix_det() {
    let m1 = Matrix::from_vec(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
    assert_eq!(m1.det(), 1)
}

#[test]
fn test_dot_product() {
    println!("Testing vector dot product");
    let v1: Vector = Vector::new(vec![0, 1, 0]);
    let v2: Vector = Vector::new(vec![1, 0, 0]);

    assert_eq!(384.0, v1.dot(&v2));
    println!("Success!")
}

#[test]
fn test_vector_add() {
    println!("Testing vector adding");
    let v1: Vector = Vector::new(vec![123, 123, 0]);
    let v2: Vector = Vector::new(vec![0, 123, 123]);

    assert_eq!(Vector::new(vec![123, 246, 123]), v1 + v2);
    println!("Success!")
}

#[test]
fn test_vector_scalar() {
    println!("Testing vector scaling");
    let v1 = Vector::new(vec![1, 1, 1]);
    let scalar = Scalar::new(3);

    assert_eq!(Vector::new(vec![3, 3, 3]), v1 * scalar);
    println!("Success!")
}

#[test]
fn test_vector_subtraction() {
    println!("Testing vector subtraction!");
    let v1 = Vector::new(vec![10, 10, 10]);
    let v2 = Vector::new(vec![5, 5, 5]);

    assert_eq!(Vector::new(vec![5, 5, 5]), v1 - v2);
    println!("Success!")
}