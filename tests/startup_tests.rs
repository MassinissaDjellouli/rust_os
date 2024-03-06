#![test_runner(rust_os::test_runner)]
#[test_case]
fn testing_test_module(){
    assert_eq!(1,1);
}
#[test_case]
fn testing_multi_line_print(){
    for i in [0..2000]{
        rust_os::println!("TEST");
    }
}
