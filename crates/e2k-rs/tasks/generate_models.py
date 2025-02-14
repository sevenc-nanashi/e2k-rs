# モデルの形式については src/model.rs のコメントを参照。

import e2k.inference
import os
import numpy as np
import safetensors.numpy as safetensors

dirname = os.path.dirname(__file__)

for file, dest in [
    ("model-c2k.npz", "model-c2k.safetensors"),
    ("model-p2k.npz", "model-p2k.safetensors"),
]:
    model = e2k.inference.get_weight_path(file)
    loaded = np.load(model)
    dest = os.path.join(dirname, "../src/models", dest)
    safetensors.save_file(loaded, dest)
    print(f"Saved {file} to {dest}")
    for key in loaded.files:
        print(f"  {key}: dtype={loaded[key].dtype}, shape={loaded[key].shape}")
