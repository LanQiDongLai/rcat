#[cfg(test)]
mod file_test{
    use rcat::file::File;
    #[test]
    fn file_get_content_test1(){
        let file = File::new("tests/test.txt");
        assert_eq!(file.get_content().unwrap(), "this is a test line");
    }
    #[test]
    #[should_panic]
    fn file_get_content_test2(){
        let file = File::new("unexist_test_file.txt");
        assert_eq!(file.get_content().unwrap(), "");
    }
}
