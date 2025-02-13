# モデルの形式については src/model.rs のコメントを参照。

import e2k.inference
import os
import numpy as np

dirname = os.path.dirname(__file__)

for file, dest in [("model-c2k.npz", "c2k.e2km"), ("model-p2k.npz", "p2k.e2km")]:
    model = e2k.inference.get_weight_path(file)
    loaded = np.load(model)
    dest = os.path.join(dirname, "../src/models", dest)
    print(f"Converting {model} to {dest}")
    with open(dest, "wb") as f:
        f.write(b"E2KM")
        f.write(b"\x01")
        f.write(len(loaded.files).to_bytes(1, "little"))
        for key in loaded.files:
            f.write(key.encode())
            f.write(b"\x00")
            f.write(len(loaded[key].shape).to_bytes(1, "little"))
            for dim in loaded[key].shape:
                f.write(dim.to_bytes(4, "little"))
            # https://stackoverflow.com/a/55627146
            if loaded[key].dtype == np.int64:
                f.write(b"\x00")
                f.write(np.ascontiguousarray(loaded[key], dtype='<i8').tobytes())
                print(f"  Converted {key} as int64")
            elif loaded[key].dtype == np.float32 or loaded[key].dtype == np.float16:
                f.write(b"\x01")
                f.write(np.ascontiguousarray(loaded[key], dtype='<f4').tobytes())
                print(f"  Converted {key} as float32")
            else:
                raise ValueError(f"Unsupported dtype: {loaded[key].dtype}")
    print(f"Converted {model} to {dest}")

