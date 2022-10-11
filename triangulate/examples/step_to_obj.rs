// cargo run --example step_to_obj models/open-nurb-1.step
// cargo run --example step_to_obj models/sphere.step
// cargo run --example step_to_obj models/weird-tube.step
// cargo run --example step_to_obj models/klein-bottle.step
// cargo run --example step_to_obj models/cylinder-with-holes.step

use clap::{Arg, App};

use triangulate::triangulate::triangulate;
use step::step_file::StepFile;
use std::io::BufWriter;

fn get_current_working_dir() -> String {
    let res = std::env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("Current directory: {}", get_current_working_dir());

    let matches = App::new("step_to_obj")
        .author("Olivier Giniaux <oginiaux@gmail.com>")
        .about("Converts a STEP file to an OBJ file")
        .arg(Arg::with_name("input")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input = matches.value_of("input")
        .expect("Could not get input file");

    let start = std::time::SystemTime::now();
    let data = std::fs::read(input)?;
    let flat = StepFile::strip_flatten(&data);
    let entities = StepFile::parse(&flat);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("Loaded + parsed in {:?}", since_the_epoch);

    let start = std::time::SystemTime::now();
    let (mesh, _stats) = triangulate(&entities);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("Triangulated in {:?}", since_the_epoch);

    let mut output = std::path::PathBuf::from(input);
    output.set_extension("obj");

    let output_file = std::fs::File::create(&output).unwrap();
    let mut writer = BufWriter::new(output_file);

    let start = std::time::SystemTime::now();
    nanomesh::io::obj::write(&mesh, &mut writer);
    let end = std::time::SystemTime::now();
    let since_the_epoch = end.duration_since(start)
        .expect("Time went backwards");
    println!("File written in {:?}", since_the_epoch);

    Ok(())
}
