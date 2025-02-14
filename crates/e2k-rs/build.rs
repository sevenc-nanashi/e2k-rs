use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for file in [
        "./src/models/model-c2k.safetensors.br",
        "./src/models/model-p2k.safetensors.br",
    ] {
        let model = root.join(file);
        let tmp_dest = root.join(file.replace(".br", ".tmp"));
        let dest = root.join(file.replace(".br", ""));
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
