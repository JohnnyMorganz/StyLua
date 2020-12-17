use luafmt_lib::format_code;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn test_folder<P: AsRef<Path>>(folder: P) {
    for entry in fs::read_dir(folder).expect("couldn't read directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        let input = fs::read_to_string(path.join("input.lua")).expect("couldn't read input.lua");

        let formatted_code = match format_code(&input) {
            Ok(code) => code,
            Err(error) => {
                panic!("error formatting {}: {}", path.display(), error)
            }
        };

        let output_path = path.join("output.lua");

        if let Ok(expected_output) = fs::read_to_string(&output_path) {
            assert_eq!(formatted_code, expected_output);
        } else {
            let mut file = File::create(&output_path).expect("couldn't write output file");
            file.write_all(formatted_code.as_bytes())
                .expect("couldn't write output file");
        }
    }
}

#[test]
fn test_examples_folder() {
    test_folder("./tests/files");
}
