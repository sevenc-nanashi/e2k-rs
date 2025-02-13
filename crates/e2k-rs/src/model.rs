// モデルのフォーマット：
// 形式：[ヘッダー][バージョン][配列数][配列1][配列2]...
//   ヘッダー：E2KM（バイト列）
//   バージョン：u8（現在：1）
//   配列数：u8
//   配列：[名前][形状][種別][要素数][データ]
//     名前：文字列（Null Terminated）
//     形状：[次元数][u32x次元数]
//       次元数：u8
//     種別：u8（0：u64、1：f32、2：f64）
//     データ：バイト列（Little Endian）

use std::collections::HashMap;

use ndarray::Dimension;

#[derive(Debug)]
pub(crate) struct Model {
    contents: HashMap<String, ModelArray>,
}

struct ModelArray {
    shape: Vec<u32>,
    data: ModelArrayData,
}

impl std::fmt::Debug for ModelArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelArray")
            .field("shape", &self.shape)
            .finish_non_exhaustive()
    }
}

pub(crate) enum ModelArrayData {
    U64(Vec<u64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

pub(crate) trait FromModelArrayDataElement {
    fn from_model_array_data_element(data: &ModelArrayData, index: usize) -> Option<Self>
    where
        Self: Sized;
}

#[duplicate::duplicate_item(
    int_type variant;
    [u64]    [ModelArrayData::U64];
    [f32]    [ModelArrayData::F32];
)]
impl FromModelArrayDataElement for int_type {
    fn from_model_array_data_element(data: &ModelArrayData, index: usize) -> Option<Self> {
        match data {
            variant(data) => data.get(index).copied(),
            _ => None,
        }
    }
}
impl FromModelArrayDataElement for f64 {
    fn from_model_array_data_element(data: &ModelArrayData, index: usize) -> Option<Self> {
        match data {
            ModelArrayData::F64(data) => data.get(index).copied(),
            ModelArrayData::F32(data) => data.get(index).copied().map(f64::from),
            _ => None,
        }
    }
}

impl Model {
    pub fn new(bytes: &[u8]) -> Self {
        if bytes.len() < 5 {
            panic!("Model: too short");
        }
        let mut bytes = bytes;
        let header = bytes.get(0..4).unwrap();
        if header != b"E2KM" {
            panic!("Model: invalid header");
        }
        bytes = bytes.get(4..).unwrap();

        let (version, bytes) = read_u8(bytes);

        if version != 1 {
            panic!("Model: invalid version");
        }

        let (array_count, bytes) = read_u8(bytes);

        if array_count == 0 {
            panic!("Model: invalid array count");
        }

        let mut contents = HashMap::new();
        let mut outer_bytes = bytes;
        for _ in 0..array_count {
            let (name, bytes) = read_string(outer_bytes);
            let (shape, bytes) = read_u8(bytes);
            let (shape, bytes) = read_u32s(bytes, shape as usize);
            let (kind, bytes) = read_u8(bytes);
            let num_elements = shape.iter().product::<u32>();
            let (data, bytes) = match kind {
                0 => {
                    let (data, bytes) = read_u64s(bytes, num_elements as usize);
                    (ModelArrayData::U64(data), bytes)
                }
                1 => {
                    let (data, bytes) = read_f32s(bytes, num_elements as usize);
                    (ModelArrayData::F32(data), bytes)
                }
                2 => {
                    let (data, bytes) = read_f64s(bytes, num_elements as usize);
                    (ModelArrayData::F64(data), bytes)
                }
                _ => panic!("Model: invalid kind"),
            };
            outer_bytes = bytes;
            contents.insert(name, ModelArray { shape, data });
        }

        Self { contents }
    }

    pub fn get_array<T: FromModelArrayDataElement + Copy, D: ndarray::Dimension>(
        &self,
        name: &str,
    ) -> Option<ndarray::Array<T, D>> {
        let array = self.contents.get(name)?;
        let shape = ndarray::IxDyn(&array.shape.iter().map(|&x| x as usize).collect::<Vec<_>>());
        let size = shape.size();
        let data = ndarray::Array::from_shape_vec(
            shape,
            (0..size)
                .map(|i| T::from_model_array_data_element(&array.data, i).unwrap())
                .collect(),
        );
        data.ok().map(|data| data.into_dimensionality().unwrap())
    }
}

fn read_string(bytes: &[u8]) -> (String, &[u8]) {
    let mut bytes = bytes;
    let mut name = Vec::new();
    loop {
        let c = bytes.first().unwrap();
        bytes = bytes.get(1..).unwrap();
        if *c == 0 {
            break;
        }
        name.push(*c);
    }
    (String::from_utf8(name).unwrap(), bytes)
}

#[duplicate::duplicate_item(
    function_name int_type;
    [read_u64]    [u64];
    [read_u32]    [u32];
    [read_f64]    [f64];
    [read_f32]    [f32];
    [read_u8]     [u8];
)]
fn function_name(bytes: &[u8]) -> (int_type, &[u8]) {
    let (data, bytes) = bytes.split_at(std::mem::size_of::<int_type>());
    (int_type::from_le_bytes(data.try_into().unwrap()), bytes)
}

#[duplicate::duplicate_item(
    function_name inner_name  int_type;
    [read_u64s]   [read_u64]  [u64];
    [read_u32s]   [read_u32]  [u32];
    [read_f64s]   [read_f64]  [f64];
    [read_f32s]   [read_f32]  [f32];
)]
fn function_name(bytes: &[u8], count: usize) -> (Vec<int_type>, &[u8]) {
    let mut bytes = bytes;
    let mut data = Vec::new();
    for _ in 0..count {
        let (value, rest) = inner_name(bytes);
        data.push(value);
        bytes = rest;
    }
    (data, bytes)
}
