use cpp_interop::hello;

fn main() {
    println!("Hello, world!");

    let mut working_buffer = vec![0; 1024];

    unsafe {
        let written = cpp_interop::write(&mut working_buffer) as usize;

        assert!(written > 0 && written < working_buffer.len());

        assert_eq!(cpp_interop::read(&mut working_buffer), 0);
    }
}
