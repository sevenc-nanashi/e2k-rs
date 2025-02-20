use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/models/model-c2k.safetensors");
    println!("cargo:rerun-if-changed=src/models/model-p2k.safetensors");
    println!("cargo:rerun-if-changed=src/models/model-c2k.safetensors.br");
    println!("cargo:rerun-if-changed=src/models/model-p2k.safetensors.br");
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    for (raw_file, compressed_file, url) in [
        (
            "./src/models/model-c2k.safetensors",
            "./src/models/model-c2k.safetensors.br",
            "https://github.com/Patchethium/e2k/releases/download/0.2.0/model-c2k.safetensors",
        ),
        (
            "./src/models/model-p2k.safetensors",
            "./src/models/model-p2k.safetensors.br",
            "https://github.com/Patchethium/e2k/releases/download/0.2.0/model-p2k.safetensors",
        ),
    ] {
        let raw_model = root.join(raw_file);
        let tmp_dest = root.join(format!("{}.tmp", compressed_file));
        if !raw_model.exists() {
            download_model(&tmp_dest, url);
            std::fs::rename(&tmp_dest, &raw_model).unwrap();
        }

        let compressed_model = root.join(compressed_file);
        if !compressed_model.exists() {
            let mut raw_model_file = std::fs::File::open(&raw_model).unwrap();
            let mut compressed_model_file = std::fs::File::create(&tmp_dest).unwrap();

            brotli::BrotliCompress(
                &mut raw_model_file,
                &mut compressed_model_file,
                &brotli::enc::BrotliEncoderParams {
                    quality: 11,
                    ..Default::default()
                },
            )
            .unwrap();
        }
    }
}

fn download_model(model: &std::path::Path, url: &str) {
    let resp = ureq::get(url).call().unwrap();
    let mut file = std::fs::File::create(model).unwrap();
    std::io::copy(&mut resp.into_body().into_reader(), &mut file).unwrap();
}
