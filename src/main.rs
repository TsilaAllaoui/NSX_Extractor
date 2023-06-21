use std::fs;
use std::process::Command;

fn main() {
    // Extract NSP/XCI
    // let mut output = Command::new("nstool.exe")
    //     .args(["-x", "tmp", "rom.nsp"])
    //     .output()
    //     .expect("Failed to execute command");

    // Get CNMT Layout file name
    let mut _cnmt_layout = String::new();
    let mut files = fs::read_dir("tmp").unwrap();
    for file in files {
        let name = file.unwrap().file_name().into_string().unwrap();
        if name.find(".cnmt.nca") != None {
            _cnmt_layout = name;
            break;
        }
    }

    // Extract CNMT Layout file
    let mut a = format!("tmp/{}", &_cnmt_layout);
    let mut output = Command::new("nstool.exe")
        .args(["-x", "tmp/cnmt", &a])
        .output()
        .expect("Failed to execute command");

    // Reading CNMT Layout file
    files = fs::read_dir("tmp/cnmt/0").unwrap();
    for file in files {
        let name = file.unwrap().file_name().into_string().unwrap();
        if name.find(".cnmt") != None {
            let n = format!("tmp/cnmt/0/{}", &name);
            output = Command::new("nstool.exe")
                .args(["-x", "tmp/cnmt/0", &n])
                .output()
                .expect("Can't extract file");
            break;
        }
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));

    let mut s = String::from_utf8_lossy(&output.stdout).to_string();
    let upper = s
        .find(
            "Control (3)
        \n      Id:           ",
        )
        .expect("position not found");
    let lower = s.find("\n      Size").expect("position not found");
    let metadata_filename: String = s.chars().skip(upper).take(lower).collect();
    print!("Metadata file name: {}", metadata_filename);
}
