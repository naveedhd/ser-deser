use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug)]
enum Degree {
    BE,
    MS,
}

#[derive(Serialize, Deserialize, Debug)]
struct Education {
    degree: Degree,
    major: String
}

#[derive(Serialize, Deserialize, Debug)]
struct JobInfo {
    title: String,
    location: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    age: usize,
    weight: f64,
    education: Vec<Education>,
    experience: HashMap<String, JobInfo>,
}

fn main() {
    let file = File::open("../../config/example.json").unwrap();
    let reader = BufReader::new(file);
    let mut config: Config = serde_json::from_reader(reader).unwrap();
    println!("{:#?}", config);

    config.age = 31;
    config.weight = 82.1;

    let masters = Education { degree: Degree::MS, major: "Mechanical Engineering".to_string() };
    config.education.push(masters);

    let current_job = JobInfo { title: "Senior Software Engineer".to_string(), location: "Berlin, Germany".to_string() };
    config.experience.insert("HERE Technologies".to_string(), current_job);

    println!("{:#?}", config);

    let file = File::create("../../config/rust-example.json").unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &config).unwrap();
}
