use compiler_tools::files::Filenames;
use std::path::Path;

#[test]
fn test_eq() {
    let path_a = Path::new("./Cargo.toml");
    let path_b = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let a = filenames.filename(&path_a).expect("Expected success");
    let b = filenames.filename(&path_b).expect("Expected success");

    assert_eq!(a, b)
}

#[test]
fn test_ne() {
    let path_a = Path::new("./src/symbol.rs");
    let path_b = Path::new("./src/files.rs");
    let mut filenames = Filenames::new();
    let a = filenames.filename(&path_a).expect("Expected success");
    let b = filenames.filename(&path_b).expect("Expected success");

    assert_ne!(a, b)
}

#[test]
fn test_eq_path() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let fname = filenames.filename(&path).expect("Expected success");

    assert_eq!(&fname, Path::new("./Cargo.toml"));
    assert_eq!(Path::new("./Cargo.toml"), &fname)
}

#[test]
fn test_ne_path() {
    let path = Path::new("./Cargo.toml");
    let mut filenames = Filenames::new();
    let fname = filenames.filename(&path).expect("Expected success");

    assert_ne!(&fname, Path::new("./src/files.rs"));
    assert_ne!(Path::new("./src/files.rs"), &fname)
}


#[test]
fn test_ord_path() {
    let path = Path::new("./src/symbol.rs");
    let mut filenames = Filenames::new();
    let fname = filenames.filename(&path).expect("Expected success");

    assert!(&fname > Path::new("./src/files.rs"));
    assert!(Path::new("./src/files.rs") < &fname)
}
