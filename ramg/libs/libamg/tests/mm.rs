#[cfg(test)]
mod tests {
    use std::env;
    use libamg::io::MatrixMarketReader;
    use std::path::Path;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn check_mm_reader() {
        let cur_path = env::current_dir().unwrap();
        println!("current path : {}", cur_path.display());
        let matrices_path = Path::new(".").join("data");
        let matrix_path = matrices_path.join("iDA_SIPG_2_test_problem_2_regular_1.m.mtx");

        let mm = MatrixMarketReader::new(matrix_path.to_str().unwrap());
    
	assert_eq!(1, 2);
    }
}
