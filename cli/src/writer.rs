fn get_dir(path: impl AsRef<Path>) -> String {
    let filename = path.as_ref().file_name().unwrap().to_str().unwrap();
    path.as_ref().to_str().unwrap().replace(filename, "")
}
#[test]
fn test_get_dir() {
    let path = "./dir1/test.txt";
    assert_eq!(get_dir(path), "./dir1/".to_string());
}
