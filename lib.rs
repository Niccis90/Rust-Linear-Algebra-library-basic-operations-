pub mod matrix_ops {

    use rand::Rng;

    pub fn matrix_multiply(matrix1: &Vec<Vec<f32>>, matrix2: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        assert_eq!(matrix1[0].len(), matrix2.len(), "The number of columns in matrix1 must match the number of rows in matrix2 for matrix multiplication.");

        let rows = matrix1.len();
        let cols = matrix2[0].len();
        let mut result = vec![vec![0.0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                for k in 0..matrix1[0].len() {
                    result[i][j] += matrix1[i][k] * matrix2[k][j];
                }
            }
        }

        result
    }

    pub fn scalar_multiply(scalar: f32, matrix: &mut Vec<Vec<f32>>) {
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix[i][j] = matrix[i][j] * (scalar)
            }
        }
    }

    pub fn scalar_subtract(scalar: f32, matrix: &mut Vec<Vec<f32>>) {
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix[i][j] = matrix[i][j] - (scalar)
            }
        }
    }

    pub fn scalar_add(scalar: f32, matrix: &mut Vec<Vec<f32>>) {
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix[i][j] = matrix[i][j] + (scalar)
            }
        }
    }

    pub fn transpose(matrix: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let rows = matrix.len();
        let cols = if rows == 0 { 0 } else { matrix[0].len() };

        let mut result = vec![vec![0.0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = matrix[i][j];
            }
        }
        result
    }

    pub fn matrix_add(matrix1: Vec<Vec<f32>>, matrix2: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        assert_eq!(matrix1.len(), matrix2.len(), "The rows need to match");
        assert_eq!(
            matrix1[0].len(),
            matrix2[0].len(),
            "The columns need to match"
        );
        let mut result = vec![vec![0.0; matrix2[0].len()]; matrix1.len()];

        for i in 0..matrix1.len() {
            for j in 0..matrix1[0].len() {
                result[i][j] = matrix1[i][j] + matrix2[i][j]
            }
        }

        result
    }

    pub fn add_bias(matrix: &mut Vec<Vec<f32>>, bias: &Vec<f32>) {
        assert_eq!(
            matrix.len(),
            bias.len(),
            "Number of rows in matrix must match length of bias vector"
        );

        for (i, row) in matrix.iter_mut().enumerate() {
            for x in row.iter_mut() {
                *x += bias[i];
            }
        }
    }

    pub fn matrix_subtraction(matrix1: &Vec<Vec<f32>>, matrix2: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        assert_eq!(matrix1.len(), matrix2.len(), "The rows need to match");
        assert_eq!(
            matrix1[0].len(),
            matrix2[0].len(),
            "The columns need to match"
        );
        let mut temp = Vec::new();

        for i in 0..matrix1.len() {
            let mut temp1 = Vec::new();
            for j in 0..matrix1[0].len() {
                temp1.push(matrix1[i][j] - matrix2[i][j])
            }
            temp.push(temp1)
        }

        temp
    }

    pub fn create_random_matrix(rows: usize, cols: usize) -> Vec<Vec<f32>> {
        let mut rng = rand::thread_rng();
        let mut matrix = Vec::with_capacity(rows);

        for _ in 0..rows {
            let row: Vec<f32> = (0..cols).map(|_| rng.gen()).collect();
            matrix.push(row);
        }

        matrix
    }
}

pub mod vector_ops {
    use rand::Rng;

    pub fn vector_scalar_multiply(scalar: f32, vector: &mut Vec<f32>) {
        for i in vector.iter_mut(){
            *i *= scalar;
        }
    }

    pub fn vector_scalar_subtract(scalar: f32, vector: &mut Vec<f32>) {
        for i in vector.iter_mut(){
            *i -= scalar;
        }
    }

    pub fn vector_scalar_add(scalar: f32, vector: &mut Vec<f32>) {
        for i in vector.iter_mut(){
            *i += scalar;
        }
    }

    pub fn vector_add(vector1: &mut Vec<f32>, vector2: &mut Vec<f32>) {
        assert_eq!(
            vector1.len(),
            vector2.len(),
            "Vectors must be of the same length"
        );
        for (x,&y) in vector1.iter_mut().zip(vector2.iter()){
            *x += y;
        }
    }

    pub fn vector_subtract(vector1: &mut Vec<f32>, vector2: &mut Vec<f32>) {
        assert_eq!(
            vector1.len(),
            vector2.len(),
            "Vectors must be of the same length"
        );
        for (x,&y) in vector1.iter_mut().zip(vector2.iter()){
            *x -= y;
        }
    }

    pub fn create_random_vector(size: usize) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen()).collect()
    }
}

pub mod csv_reader {
    use csv;
    use serde::Deserialize;
    use std::error::Error;

    pub fn read_csv_to_vec(filepath: &str) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(filepath)?;
        let mut data = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let values: Vec<f32> = record
                .iter()
                .map(|field| field.parse::<f32>().unwrap())
                .collect();
            data.push(values);
        }

        Ok(data)
    }
}
