use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_EMBED_MODEL");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if std::env::var("DOCS_RS").is_ok() {
        std::fs::write("src/models/model-c2k.safetensors", "__docs_rs_placeholder__").unwrap();
        std::fs::write("src/models/model-c2k.safetensors.br", "__docs_rs_placeholder__").unwrap();
        std::fs::write("src/models/model-p2k.safetensors", "__docs_rs_placeholder__").unwrap();
        std::fs::write("src/models/model-p2k.safetensors.br", "__docs_rs_placeholder__").unwrap();
        return;
    }
    if std::env::var("CARGO_FEATURE_EMBED_MODEL") == Ok("1".to_string()) {
        download_models();
    }
}

fn download_models() {
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
        if !raw_model.exists() {
            let tmp_raw_dest = root.join(format!("{}.tmp", raw_file));
            download_model(&tmp_raw_dest, url);
            std::fs::rename(&tmp_raw_dest, &raw_model).unwrap();
        }

        let compressed_model = root.join(compressed_file);
        if !compressed_model.exists() {
            let temp_compressed_dest = root.join(format!("{}.tmp", compressed_file));
            let mut raw_model_file = std::fs::File::open(&raw_model).unwrap();
            let mut compressed_model_file = std::fs::File::create(&temp_compressed_dest).unwrap();

            brotli::BrotliCompress(
                &mut raw_model_file,
                &mut compressed_model_file,
                &brotli::enc::BrotliEncoderParams {
                    quality: 11,
                    ..Default::default()
                },
            )
            .unwrap();

            std::fs::rename(&temp_compressed_dest, &compressed_model).unwrap();
        }
    }
}

fn download_model(model: &std::path::Path, url: &str) {
    let resp = ureq::get(url).call().unwrap();
    let mut file = std::fs::File::create(model).unwrap();
    std::io::copy(&mut resp.into_body().into_reader(), &mut file).unwrap();
}
