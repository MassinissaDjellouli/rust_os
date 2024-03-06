use rust_os::vga_buf::WRITER;

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

#[test_case]
fn testing_println_output(){
    let s = "Some test string that should be printed";
    rust_os::println!("{}", s);
    for (i,c) in s.chars().enumerate(){
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_char),c);
    }
}