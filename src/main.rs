use std::{
    env,
    error::Error,
    ffi::OsString,
    process,
};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn run()->Result<(),Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.records(){
        let record = result?;
        println!("{:?}",record);
        }
        Ok(())
}

fn main(){
   if let Err(err) = run(){
        println!("error reading CSV from <stdin>: {}", err);
        process::exit(1);
    }
}