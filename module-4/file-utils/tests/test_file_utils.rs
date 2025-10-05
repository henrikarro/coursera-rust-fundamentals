use file_utils::{read_file, write_file, append_to_file};

#[test]
fn test_file_operations() {
    let test_file = "test_example.txt";

    // Test writing to a file
    write_file(test_file, "Hello, world!\n").unwrap();
    let content = read_file(test_file).unwrap();
    assert_eq!(content, "Hello, world!\n");

    // Test appending to a file
    append_to_file(test_file, "This is an appended line.").unwrap();
    let updated_content = read_file(test_file).unwrap();
    assert_eq!(updated_content, "Hello, world!\nThis is an appended line.\n");

    // Clean up
    std::fs::remove_file(test_file).unwrap();
}
