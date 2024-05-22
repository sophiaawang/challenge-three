use serde::Deserialize;
use std::env;
use std::fs;
use std::io::*;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

fn main() {
    let mut tossed_images: Vec<String> = Vec::new();

    match fs::create_dir_all("./trash") {
        Ok(_) => {}
        Err(_) => {
            print!("Could not create trash directory.");
            return;
        }
    };

    let paths = fs::read_dir("./files/").unwrap();

    for entry in paths {
        let p = entry.unwrap();
        let json_path = p.path();
        let ext = json_path.extension();

        if let Some(ext) = ext {
            let ext = ext.to_string_lossy();
            if ext == "json" {
                match below_height(&json_path, 100.0) {
                    Ok(below) => {
                        if below {
                            let image_path = get_image(&json_path);
                            if let Err(_) = toss(&image_path) {
                                let msg = "failed to toss jpeg: ";
                                msg.to_owned().push_str(
                                    image_path.clone().to_string_lossy().to_string().as_str(),
                                );
                                println!("{}", msg);
                            };
                            match image_path.file_name() {
                                Some(name) => match name.to_owned().to_str() {
                                    Some(name_str) => {
                                        tossed_images.push(name_str.to_string());
                                    }
                                    None => {}
                                },
                                None => {
                                    println!("no image filename was found")
                                }
                            }
                            if let Err(_) = toss(&json_path) {
                                let msg = "failed to toss json: ";
                                msg.to_owned().push_str(
                                    json_path.clone().to_string_lossy().to_string().as_str(),
                                );
                                println!("{}", msg);
                            };
                        }
                    }
                    Err(err) => {
                        print!("{}", err);
                    }
                }
            }
        }
    }

    if tossed_images.len() == 0 {
        print!("No files moved.");
    } else {
        print!("Files moved: ");
        println!("{:?}", tossed_images);
    }
}

/*function to get associated image*/
fn get_image(path: &std::path::Path) -> std::path::PathBuf {
    let check = path.with_extension("JPG");
    return check;
}

/*function to move a file to the trash and add to log*/
fn toss(path: &std::path::Path) -> Result<()> {
    match path.file_name() {
        Some(filename) => {
            let mut dir = env::current_exe()?;
            dir.pop();
            dir.pop();
            dir.pop();
            dir.push("trash");
            dir.push(filename);
            fs::rename(path, dir)?;
        }
        None => {
            print!("error??");
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "missing end of filename",
            ));
        }
    };
    Ok(())
}

/*check height*/
fn below_height(path: &PathBuf, height: f64) -> Result<bool> {
    let data = fs::read_to_string(path)?;
    let image_details: ImageDetails = serde_json::from_str(&data)?;
    let pix = image_details.pixhawk;
    let pos = pix.position; // "pos: piece of shit"

    let alt = pos.0.altitude_msl;

    if alt <= height {
        return Ok(true);
    }

    Ok(false)
}

#[derive(Deserialize, Debug)]
struct ImageDetails {
    csb: Option<String>,
    pixhawk: Telemetry,
}

#[derive(Deserialize, Debug)]
struct Telemetry {
    attitude: Attitude,
    position: Position,
    velocity: Velocity,
}

#[derive(Deserialize, Debug)]
struct Attitude(AttitudeData, String);

#[derive(Deserialize, Debug)]
struct AttitudeData {
    pitch: f64,
    roll: f64,
    yaw: f64,
}

#[derive(Deserialize, Debug)]
struct Position(PositionData, String);

#[derive(Deserialize, Debug)]
struct PositionData {
    altitude_msl: f64,
    altitude_rel: f64,
    point: Point,
}

#[derive(Deserialize, Debug)]
struct Point {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize, Debug)]
struct Velocity(VelocityData, String);

#[derive(Deserialize, Debug)]
struct VelocityData {
    x: f64,
    y: f64,
    z: f64,
}
