use std::fs;
use std::process::Command;

fn get_game_infos(input: String) {
    // Safeguards
    fs::read("prod.keys")
        .expect("Decryption keys not found! Add \"prod.keys\" with \"nstool.exe\"");
    fs::read("nstool.exe").expect("\"nstool.exe\" not found!");

    // Extract NSP/XCI
    Command::new("nstool.exe")
        .args(["-x", "tmp", &input])
        .output()
        .expect("Failed to extract rom file!");

    // Get CNMT Layout file name
    let mut cnmt_layout_filename = String::new();
    let mut files = fs::read_dir("tmp").unwrap();
    for file in files {
        let name = String::from(file.expect("File not found").file_name().to_string_lossy());
        if name.find(".cnmt.nca") != None {
            cnmt_layout_filename = String::from(name);
            break;
        }
    }

    // Extract CNMT Layout file
    let a = format!("tmp/{}", &cnmt_layout_filename);
    let mut output = Command::new("nstool.exe")
        .args(["-x", "tmp/cnmt", &a])
        .output()
        .expect("Failed to execute command");

    // Reading CNMT Layout file
    files = fs::read_dir("tmp/cnmt/0").expect("tmp directory not found");
    for file in files {
        let name = file
            .expect("File not found")
            .file_name()
            .into_string()
            .expect("Conversion error");
        if name.find(".cnmt") != None {
            let n = format!("tmp/cnmt/0/{}", &name);
            output = Command::new("nstool.exe")
                .args(["-x", "tmp/cnmt/0", &n])
                .output()
                .expect("Can't extract file");
            break;
        }
    }

    // Get Metadata filename
    let mut metadata_filename = String::new();
    let out = String::from_utf8_lossy(&output.stdout).to_string();
    let mut outs: Vec<&str> = out.split("\n").collect();
    for i in 1..outs.len() {
        if outs[i].find("Control") != None {
            let mut b: Vec<&str> = outs[i + 1].split(" ").collect();
            b = b.last().expect("Index error").split("\r").collect();
            metadata_filename = b.first().expect("Index error").to_string() + ".nca";
            break;
        }
    }

    // Extracting and reading metadata file
    Command::new("nstool.exe")
        .args([
            "-x",
            "tmp/metadata",
            &String::from(format!("tmp/{}", &metadata_filename)),
        ])
        .output()
        .expect("Error extracting metadata");

    output = Command::new("nstool.exe")
        .args(["tmp/metadata/0/control.nacp"])
        .output()
        .expect("Error extracting metadata");

    let mut title = String::from_utf8_lossy(&output.stdout).to_string();
    outs = title.split("\n").collect();
    for out in outs {
        if out.find("Name") != None {
            title = String::from(out.split("       ").last().expect("Index error"));
            title = String::from(&title[0..title.len()-1]);
            break;
        }
    }

    // Creating output file and copying icon
    let dir_name = title.replace(" " , "_").replace(":", "").to_string();
    fs::create_dir(format!("games/{}", dir_name)).expect("Error creating output for games");
    fs::write(format!("games/{}/{}.txt", dir_name,dir_name), format!("Game Title Name: {}",title)).expect("Can't write file");
    files = fs::read_dir("tmp/metadata/0").expect("Folder not found");
    for file in files {
        let name = file.expect("File not found").file_name().into_string().expect("Conversion error");
        if name.find(".dat") != None {
            fs::copy(format!("tmp/metadata/0/{}", name), format!("games/{}/{}.png", dir_name, name)).expect("Can't copy file");
            break;
        }
    }
}

fn main() {
    // Cleaning older temp files
    let _ = fs::remove_dir_all("tmp");
    _ = fs::remove_dir_all("games");
    fs::create_dir("games").expect("Error creating output for games");

    // Iterating files, getting rom files then extract icon and infos
    let rom_files = fs::read_dir(".").expect("Error getting rom files");
    for file in rom_files {
        let path = file.as_ref().expect("Error getting file").file_name().into_string().expect("Conversion error");
        let full_path = file.expect("File not found").path().as_path().to_string_lossy().to_string();
        if path.find(".nsp") != None || path.find(".xci") != None {
            println!("Getting infos of :{}", path);
            get_game_infos(String::from(full_path));
            _ = fs::remove_dir_all("tmp");
        }
    }
}
