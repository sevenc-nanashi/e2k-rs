use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/models/model-c2k.safetensors.br");
    println!("cargo:rerun-if-changed=src/models/model-p2k.safetensors.br");
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    for file in [
        "./src/models/model-c2k.safetensors.br",
        "./src/models/model-p2k.safetensors.br",
    ] {
        let model = root.join(file);
        let tmp_dest = root.join(file.replace(".br", ".tmp"));
        let dest = root.join(file.replace(".br", ""));
        if !model.exists() {
            download_model();
        }
        if !dest.exists() {
            let tmp_dest_file =
                std::fs::File::create(&tmp_dest).expect("Failed to create temp file");
            let model_file = std::fs::File::open(&model).expect("Failed to open model");
            brotli_decompressor::copy_from_to(model_file, tmp_dest_file)
                .expect("Failed to decompress model");
            std::fs::rename(&tmp_dest, &dest).expect("Failed to rename temp file");
        }
    }
}

fn download_model() {
    let crate_010_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("e2k-0.1.0.tgz");
    let crate_010_url = "https://static.crates.io/crates/e2k/0.1.0/download";
    if !crate_010_path.exists() {
        let crate_010_tmp_path = crate_010_path.with_extension("tmp");
        let mut crate_010 =
            std::fs::File::create(&crate_010_tmp_path).expect("Failed to create temp file");
        let resp = ureq::get(crate_010_url).call().unwrap().into_body();
        std::io::copy(&mut resp.into_reader(), &mut crate_010)
            .expect("Failed to download e2k-0.1.0.tgz");

        std::fs::rename(&crate_010_tmp_path, &crate_010_path).expect("Failed to rename temp file");
    }

    let crate_010 = std::fs::File::open(&crate_010_path).expect("Failed to open e2k-0.1.0.tgz");
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(crate_010));
    let mut unpacked = 0;
    for mut entry in archive.entries().expect("Failed to read archive").flatten() {
        let path = entry.path().expect("Failed to get path");
        if !path
            .file_name()
            .map(|x: &std::ffi::OsStr| x.to_string_lossy().ends_with(".safetensors.br"))
            .unwrap_or(false)
        {
            continue;
        }
        let path = PathBuf::from(path);
        let path = path
            .strip_prefix("e2k-0.1.0")
            .expect("Failed to strip prefix");
        let dest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(path);
        if dest.exists() {
            continue;
        }
        unpacked += 1;
        entry.unpack(&dest).expect("Failed to unpack entry");
    }
    assert_eq!(unpacked, 2);
}
