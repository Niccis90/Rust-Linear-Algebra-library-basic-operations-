# Rust-Linear-Algebra-library-basic-operations-
This is my self made linear algebra library, that has basic features.

Rust Matrix and Vector Operations Library

-Overview:

This library is a comprehensive collection of mathematical operations for matrix and vector manipulation in Rust. It provides functionality for matrix multiplication, scalar operations on matrices and vectors, matrix transposition, matrix addition and subtraction, bias addition to matrices, as well as utilities for generating random matrices and vectors. Additionally, it includes a CSV reader for converting CSV data into a matrix.

Modules:

-matrix_ops

This module includes functions for various matrix operations:

matrix_multiply: Multiply two matrices.
scalar_multiply: Multiply each element of a matrix by a scalar.
scalar_subtract: Subtract a scalar from each element of a matrix.
scalar_add: Add a scalar to each element of a matrix.
transpose: Transpose a matrix.
matrix_add: Add two matrices.
add_bias: Add a bias vector to each row of a matrix.
matrix_subtraction: Subtract one matrix from another.
create_random_matrix: Generate a matrix with random elements.

-vector_ops

This module includes functions for vector operations:

vector_scalar_multiply: Multiply each element of a vector by a scalar.
vector_scalar_subtract: Subtract a scalar from each element of a vector.
vector_scalar_add: Add a scalar to each element of a vector.
vector_add: Add two vectors.
vector_subtract: Subtract one vector from another.
create_random_vector: Generate a vector with random elements.

-csv_reader

This module provides a function to read CSV files into a vector of vectors:

read_csv_to_vec: Read data from a CSV file into a Vec<Vec<f32>>.
