use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

const ADDRESS: &str = "8.8.8.8";

fn write_file_test(path: &str) -> Result<(), io::Error> {
    let mut _write_file = File::create(path)?;
    let _data = "Writing some stuff in a file";
    _write_file.write_all(_data.as_bytes())?;
    println!("W: {path} < {_data}");
    Ok(())
}

fn read_file_test(path: &str) -> Result<(), io::Error> {
    let mut _read_file = File::open(path)?;
    let mut _contents = String::new();
    _read_file.read_to_string(&mut _contents)?;
    println!("R: {path} > {_contents}");
    Ok(())
}

fn main() {
    // test network
    match ping::new(ADDRESS.parse().unwrap()).send() {
        Ok(_) => println!("Successfully pinged {ADDRESS}"),
        Err(e) => println!("Could not ping {ADDRESS} => {e}"),
    }

    let _path_str: &str = "/tmp/created.txt";
    let _res = write_file_test(_path_str);
    match _res {
        Ok(_) => {}
        Err(e) => println!("Error; cannot write to {_path_str} => {e}"),
    }

    let _path_str: &str = "/tmp/read.txt";
    // attempt to read a file somewhere on the system
    let _res = read_file_test(_path_str);
    match _res {
        Ok(_) => {}
        Err(e) => println!("Error; cannot read from {_path_str} => {e}"),
    }
}
