use itertools::Itertools;
use ndarray::prelude::*;

pub(crate) fn sigmoid_1d(x: Array1<f32>) -> Array1<f32> {
    x.map(|x| 1.0 / (1.0 + (-x).exp()))
}

pub(crate) fn matmul_3d<T>(a: &ndarray::Array3<T>, b: &ndarray::Array3<T>) -> ndarray::Array3<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Default
        + num_traits::identities::Zero
        + Copy
        + std::ops::AddAssign,
{
    let mut result = ndarray::Array3::zeros((a.shape()[0], a.shape()[1], b.shape()[2]));
    for i in 0..a.shape()[0] {
        for j in 0..a.shape()[1] {
            for k in 0..b.shape()[2] {
                for l in 0..a.shape()[2] {
                    result[[i, j, k]] += a[[i, j, l]] * b[[i, l, k]];
                }
            }
        }
    }
    result
}

pub(crate) fn split_ndarray<T, D: ndarray::Dimension>(
    array: &ndarray::Array<T, D>,
    n: usize,
    axis: ndarray::Axis,
) -> Vec<ndarray::ArrayView<T, D>> {
    if array.shape()[axis.index()] % n != 0 {
        panic!("Cannot split the array evenly");
    }
    array
        .axis_chunks_iter(axis, array.shape()[axis.index()] / n)
        .collect()
}

macro_rules! split_ndarray {
    ($array:expr, $n:expr, $axis:expr) => {{
        let split = split_ndarray($array, $n, $axis);
        split.into_iter().collect_tuple().unwrap()
    }};
}
macro_rules! split_ndarray_owned {
    ($array:expr, $n:expr, $axis:expr) => {{
        let split = split_ndarray($array, $n, $axis);
        split
            .into_iter()
            .map(|x| x.to_owned())
            .collect_tuple()
            .unwrap()
    }};
}

#[derive(Debug)]
pub(crate) struct Linear {
    weight: ndarray::Array2<f32>,
    bias: ndarray::Array1<f32>,
}

impl Linear {
    pub fn new(weight: ndarray::Array2<f32>, bias: ndarray::Array1<f32>) -> Self {
        Self { weight, bias }
    }
    pub fn forward_2d(&self, input: &ndarray::ArrayView2<f32>) -> ndarray::Array2<f32> {
        let output = input.dot(&self.weight.t());
        output + &self.bias
    }
    pub fn forward_1d(&self, input: &ndarray::ArrayView1<f32>) -> ndarray::Array1<f32> {
        let output = input.dot(&self.weight.t());
        output + &self.bias
    }
}

#[derive(Debug)]
pub(crate) struct Embedding {
    weight: ndarray::Array2<f32>,
}

impl Embedding {
    pub fn new(weight: ndarray::Array2<f32>) -> Self {
        Self { weight }
    }
    pub fn forward(&self, input: &ndarray::Array1<usize>) -> ndarray::Array2<f32> {
        ndarray::stack(
            ndarray::Axis(0),
            &input
                .iter()
                .map(|&idx| self.weight.index_axis(ndarray::Axis(0), idx))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
}

/// Multi-Head Attention Layer
#[derive(Debug)]
pub(crate) struct Mha {
    q_proj: Linear,
    k_proj: Linear,
    v_proj: Linear,
    out_proj: Linear,
    n_heads: usize,
    scale: f32,
}

impl Mha {
    pub(crate) fn new(
        in_proj_weight: ndarray::Array2<f32>,
        in_proj_bias: ndarray::Array1<f32>,
        out_proj_weight: ndarray::Array2<f32>,
        out_proj_bias: ndarray::Array1<f32>,
        num_heads: usize,
    ) -> Self {
        let (q_w, k_w, v_w) = split_ndarray!(&in_proj_weight, 3, ndarray::Axis(0));
        let (q_b, k_b, v_b) = split_ndarray!(&in_proj_bias, 3, ndarray::Axis(0));
        let dim = q_w.shape()[q_w.ndim() - 1];
        let q_proj = Linear::new(q_w.to_owned(), q_b.to_owned());
        let k_proj = Linear::new(k_w.to_owned(), k_b.to_owned());
        let v_proj = Linear::new(v_w.to_owned(), v_b.to_owned());
        let out_proj = Linear::new(out_proj_weight, out_proj_bias);
        let scale = (dim as f32).sqrt();
        Self {
            q_proj,
            k_proj,
            v_proj,
            out_proj,
            n_heads: num_heads,
            scale,
        }
    }

    pub(crate) fn forward(
        &self,
        query: &ndarray::ArrayView2<f32>,
        key: &ndarray::ArrayView2<f32>,
        value: &ndarray::ArrayView2<f32>,
    ) -> ndarray::Array2<f32> {
        let q = self.q_proj.forward_2d(query);
        let k = self.k_proj.forward_2d(key);
        let v = self.v_proj.forward_2d(value);
        let q = split_ndarray(&q, self.n_heads, ndarray::Axis(q.ndim() - 1));
        let q = ndarray::stack(ndarray::Axis(0), &q).unwrap();
        let k = split_ndarray(&k, self.n_heads, ndarray::Axis(k.ndim() - 1));
        let k = ndarray::stack(ndarray::Axis(0), &k).unwrap();
        let v = split_ndarray(&v, self.n_heads, ndarray::Axis(v.ndim() - 1));
        let v = ndarray::stack(ndarray::Axis(0), &v).unwrap();
        let mut transposed = k.clone();
        transposed.swap_axes(2, 1);
        let attn = matmul_3d(&q, &transposed);
        let attn = attn / self.scale;
        let attn = attn.exp();
        let attn_sum = attn
            .sum_axis(ndarray::Axis(attn.ndim() - 1))
            .insert_axis(ndarray::Axis(attn.ndim() - 1));
        let attn = attn / attn_sum;
        let mut output = matmul_3d(&attn, &v);
        output.swap_axes(0, 1);
        let output = output
            .to_shape((output.shape()[0], output.shape()[1] * output.shape()[2]))
            .unwrap();
        self.out_proj.forward_2d(&output.view())
    }
}

#[derive(Debug)]
pub(crate) struct GruCell {
    ih: Linear,
    hh: Linear,
}

impl GruCell {
    pub(crate) fn new(
        weight_ih: ndarray::Array2<f32>,
        weight_hh: ndarray::Array2<f32>,
        bias_ih: ndarray::Array1<f32>,
        bias_hh: ndarray::Array1<f32>,
    ) -> Self {
        let ih = Linear::new(weight_ih, bias_ih);
        let hh = Linear::new(weight_hh, bias_hh);
        Self { ih, hh }
    }

    pub(crate) fn forward(
        &self,
        input: &ndarray::ArrayView1<f32>,
        hidden: &Option<ndarray::ArrayView1<f32>>,
    ) -> ndarray::Array1<f32> {
        let hidden = hidden.map_or_else(
            || ndarray::Array1::zeros((self.hh.weight.shape()[self.hh.weight.ndim() - 1],)),
            |x| x.to_owned(),
        );
        let rzn_ih = self.ih.forward_1d(input);
        let rzn_hh = self.hh.forward_1d(&hidden.view());

        let (rz_ih, n_ih) = rzn_ih
            .view()
            .split_at(ndarray::Axis(0), rzn_ih.shape()[rzn_ih.ndim() - 1] * 2 / 3);
        let (rz_hh, n_hh) = rzn_hh
            .view()
            .split_at(ndarray::Axis(0), rzn_hh.shape()[rzn_hh.ndim() - 1] * 2 / 3);

        let rz = sigmoid_1d(rz_ih.to_owned() + rz_hh.to_owned());
        let (r, z) = split_ndarray_owned!(&rz, 2, ndarray::Axis(rz.ndim() - 1));

        let n = (n_ih.to_owned() + r * n_hh.to_owned()).mapv(|x| x.tanh());
        (1.0 - z.clone()) * n + z * hidden
    }
}

#[derive(Debug)]
pub(crate) struct Gru {
    cell: GruCell,
    reverse: bool,
}

impl Gru {
    pub(crate) fn new(cell: GruCell, reverse: bool) -> Self {
        Self { cell, reverse }
    }

    pub(crate) fn forward(
        &self,
        input: &ndarray::ArrayView2<f32>,
        hidden: Option<ndarray::ArrayView1<f32>>,
    ) -> (ndarray::Array2<f32>, ndarray::Array1<f32>) {
        let mut hidden = hidden.map(|x| x.to_owned());
        let input = if self.reverse {
            input.slice(s![..; -1, ..]).to_owned()
        } else {
            input.to_owned()
        };
        let mut outputs = Vec::with_capacity(input.shape()[0]);
        for i in 0..input.shape()[0] {
            hidden = Some(match &hidden {
                Some(h) => self
                    .cell
                    .forward(&input.index_axis(ndarray::Axis(0), i), &Some(h.view())),
                None => self
                    .cell
                    .forward(&input.index_axis(ndarray::Axis(0), i), &None),
            });
            outputs.push(hidden.clone().unwrap());
        }
        let mut outputs = ndarray::stack(
            ndarray::Axis(0),
            &outputs.iter().map(|o| o.view()).collect_vec(),
        )
        .unwrap();
        if self.reverse {
            outputs = outputs.slice(s![..; -1, ..]).to_owned();
        }
        (outputs, hidden.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_linear() {
        let linear = Linear::new(array![[1.0, 2.0], [3.0, 4.0]], array![5.0, 6.0]);
        let input = array![[7.0, 8.0], [9.0, 10.0]];
        let output = linear.forward_2d(&input.view());
        assert_eq!(output, array![[28.0, 59.0], [34.0, 73.0]]);
    }

    #[test]
    fn test_embedding() {
        let embedding = Embedding::new(array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
        let input = array![1, 2, 0];
        let output = embedding.forward(&input);
        assert_eq!(output, array![[3.0, 4.0], [5.0, 6.0], [1.0, 2.0]]);
    }

    #[test]
    fn test_split_ndarray() {
        let array = array![[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]];
        let split = split_ndarray(&array, 2, ndarray::Axis(0));
        assert_eq!(split.len(), 2);
        assert_eq!(split[0], array![[1, 2, 3], [4, 5, 6]]);
        assert_eq!(split[1], array![[7, 8, 9], [10, 11, 12]]);
    }

    #[test]
    fn test_matmul_3d() {
        // In : import numpy as np
        // In : np.matmul(
        // ...:     np.array(
        // ...:         [[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], [[7.0, 8.0, 9.0], [10.0, 11.0, 12.0]]]
        // ...:     ),
        // ...:     np.array(
        // ...:         [
        // ...:             [[13.0, 14.0], [15.0, 16.0], [17.0, 18.0]],
        // ...:             [[19.0, 20.0], [21.0, 22.0], [23.0, 24.0]],
        // ...:         ]
        // ...:     ),
        // ...: )
        // Out: array([[[94.0, 100.0], [229.0, 244.0]], [[508.0, 532.0], [697.0, 730.0]]])
        let a = array![
            [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
            [[7.0, 8.0, 9.0], [10.0, 11.0, 12.0]],
        ];
        let b = array![
            [[13.0, 14.0], [15.0, 16.0], [17.0, 18.0]],
            [[19.0, 20.0], [21.0, 22.0], [23.0, 24.0]],
        ];
        let result = matmul_3d(&a, &b);
        assert_eq!(
            result,
            array![[[94., 100.], [229., 244.]], [[508., 532.], [697., 730.]]]
        );
    }
}
