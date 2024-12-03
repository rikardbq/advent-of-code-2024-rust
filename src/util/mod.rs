use std::{fs::File, io::Read};

pub fn get_input_contents(location: &str) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();

    let _ = (match File::open(location) {
        Ok(f) => f,
        Err(err) => {
            panic!("{:?}", err)
        }
    })
    .read_to_end(&mut buffer);

    buffer
}
