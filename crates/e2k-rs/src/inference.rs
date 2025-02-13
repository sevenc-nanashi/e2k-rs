use crate::{constants, layers};
use educe::Educe;
use rand::prelude::*;
use std::{collections::HashMap, hash::Hash};

/// デコードに使うアルゴリズム。
///
/// [StrategyTopK] 、 [StrategyTopP] も参照。
#[derive(Debug)]
pub enum Strategy {
    Greedy,
    TopK(StrategyTopK),
    TopP(StrategyTopP),
}

/// Top-Kアルゴリズムのパラメータ。
#[derive(Debug, Educe)]
#[educe(Default)]
pub struct StrategyTopK {
    #[educe(Default(expression = 3))]
    pub k: usize,
}

/// Top-Pアルゴリズムのパラメータ。
#[derive(Debug, Educe)]
#[educe(Default)]
pub struct StrategyTopP {
    #[educe(Default(expression = 0.9))]
    pub top_p: f64,
    #[educe(Default(expression = 1.0))]
    pub temperature: f64,
}

/// 英単語 -> カタカナの変換。
pub struct C2k {
    inner: BaseE2k<char, char>,
}

impl std::fmt::Debug for C2k {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("C2k").finish()
    }
}

impl C2k {
    /// 新しいインスタンスを生成する。
    ///
    /// # Arguments
    ///
    /// max_len: 読みの最大長。
    pub fn new(max_len: usize) -> Self {
        static MODEL: &[u8] = include_bytes!("./models/c2k.e2km");
        let weights = crate::model::Model::new(MODEL);
        let inner = BaseE2k {
            s2s: S2s::new(weights, max_len),
            in_table: constants::ASCII_ENTRIES
                .iter()
                .enumerate()
                .map(|(i, &c)| (c.chars().next().unwrap(), i))
                .collect(),
            out_table: constants::KANAS
                .iter()
                .enumerate()
                .map(|(i, &c)| (i, c.chars().next().unwrap()))
                .collect(),
        };
        Self { inner }
    }

    /// 推論を行う。
    pub fn infer(&self, input: &str) -> String {
        let input = input.chars().collect::<Vec<_>>();
        self.inner.infer(&input).into_iter().collect()
    }

    /// アルゴリズムを設定する。
    pub fn set_decode_strategy(&mut self, strategy: Strategy) {
        self.inner.set_decode_strategy(strategy);
    }
}

/// 発音 -> カタカナの変換。
pub struct P2k {
    inner: BaseE2k<String, char>,
}

impl std::fmt::Debug for P2k {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P2k").finish()
    }
}

impl P2k {
    /// 新しいインスタンスを生成する。
    ///
    /// # Arguments
    ///
    /// max_len: 読みの最大長。
    pub fn new(max_len: usize) -> Self {
        static MODEL: &[u8] = include_bytes!("./models/p2k.e2km");
        let weights = crate::model::Model::new(MODEL);
        let inner = BaseE2k {
            s2s: S2s::new(weights, max_len),
            in_table: constants::EN_PHONES
                .iter()
                .enumerate()
                .map(|(i, &c)| (c.to_string(), i))
                .collect(),
            out_table: constants::KANAS
                .iter()
                .enumerate()
                .map(|(i, &c)| (i, c.chars().next().unwrap()))
                .collect(),
        };
        Self { inner }
    }

    /// 推論を行う。
    ///
    /// # Arguments
    ///
    /// input: CMUDictの発音記号。
    pub fn infer(&self, input: &[&str]) -> String {
        self.inner
            .infer(&input.iter().map(|&s| s.to_string()).collect::<Vec<_>>())
            .into_iter()
            .collect()
    }

    /// アルゴリズムを設定する。
    pub fn set_decode_strategy(&mut self, strategy: Strategy) {
        self.inner.set_decode_strategy(strategy);
    }
}

struct BaseE2k<I: Hash + Eq, O: Clone> {
    s2s: S2s,
    in_table: HashMap<I, usize>,
    out_table: HashMap<usize, O>,
}

impl<I: Hash + Eq, O: Clone> BaseE2k<I, O> {
    fn infer(&self, input: &[I]) -> Vec<O> {
        let source = [constants::SOS_IDX]
            .into_iter()
            .chain(input.iter().filter_map(|c| self.in_table.get(c).copied()))
            .chain([constants::EOS_IDX]);
        let source = ndarray::Array1::from_iter(source);
        let target = self.s2s.forward(&source);
        target
            .iter()
            .skip(1)
            .take_while(|&&x| x != constants::EOS_IDX)
            .map(|&x| self.out_table[&x].clone())
            .collect()
    }

    fn set_decode_strategy(&mut self, strategy: Strategy) {
        self.s2s.strategy = strategy;
    }
}

struct S2s {
    e_emb: layers::Embedding,
    k_emb: layers::Embedding,
    encoder: layers::Gru,
    encoder_reverse: layers::Gru,
    encoder_fc: layers::Linear,
    pre_decoder: layers::Gru,
    post_decoder: layers::Gru,
    attn: layers::Mha,
    fc: layers::Linear,
    max_len: usize,

    strategy: Strategy,
}

impl S2s {
    fn new(weights: crate::model::Model, max_len: usize) -> Self {
        let e_emb = layers::Embedding::new(
            weights
                .get_array::<f64, ndarray::Ix2>("e_emb.weight")
                .unwrap(),
        );
        let k_emb = layers::Embedding::new(weights.get_array("k_emb.weight").unwrap());
        let encoder = layers::Gru::new(
            layers::GruCell::new(
                weights.get_array("encoder.weight_ih_l0").unwrap(),
                weights.get_array("encoder.weight_hh_l0").unwrap(),
                weights.get_array("encoder.bias_ih_l0").unwrap(),
                weights.get_array("encoder.bias_hh_l0").unwrap(),
            ),
            false,
        );
        let encoder_reverse = layers::Gru::new(
            layers::GruCell::new(
                weights.get_array("encoder.weight_ih_l0_reverse").unwrap(),
                weights.get_array("encoder.weight_hh_l0_reverse").unwrap(),
                weights.get_array("encoder.bias_ih_l0_reverse").unwrap(),
                weights.get_array("encoder.bias_hh_l0_reverse").unwrap(),
            ),
            true,
        );
        let encoder_fc = layers::Linear::new(
            weights.get_array("encoder_fc.0.weight").unwrap(),
            weights.get_array("encoder_fc.0.bias").unwrap(),
        );
        let pre_decoder = layers::Gru::new(
            layers::GruCell::new(
                weights.get_array("pre_decoder.weight_ih_l0").unwrap(),
                weights.get_array("pre_decoder.weight_hh_l0").unwrap(),
                weights.get_array("pre_decoder.bias_ih_l0").unwrap(),
                weights.get_array("pre_decoder.bias_hh_l0").unwrap(),
            ),
            false,
        );
        let post_decoder = layers::Gru::new(
            layers::GruCell::new(
                weights.get_array("post_decoder.weight_ih_l0").unwrap(),
                weights.get_array("post_decoder.weight_hh_l0").unwrap(),
                weights.get_array("post_decoder.bias_ih_l0").unwrap(),
                weights.get_array("post_decoder.bias_hh_l0").unwrap(),
            ),
            false,
        );
        let attn = layers::Mha::new(
            weights.get_array("attn.in_proj_weight").unwrap(),
            weights.get_array("attn.in_proj_bias").unwrap(),
            weights.get_array("attn.out_proj.weight").unwrap(),
            weights.get_array("attn.out_proj.bias").unwrap(),
            4,
        );
        let fc = layers::Linear::new(
            weights.get_array("fc.weight").unwrap(),
            weights.get_array("fc.bias").unwrap(),
        );
        let strategy = Strategy::Greedy;
        Self {
            e_emb,
            k_emb,
            encoder,
            encoder_reverse,
            encoder_fc,
            pre_decoder,
            post_decoder,
            attn,
            fc,
            max_len,
            strategy,
        }
    }

    fn greedy(&self, step_dec: &ndarray::ArrayView1<f64>) -> usize {
        let max = *step_dec
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let argmax = step_dec.iter().position(|&x| x == max).unwrap();
        argmax
    }

    fn top_k(&self, step_dec: &ndarray::ArrayView1<f64>, k: usize) -> usize {
        let step_dec = step_dec.to_vec();
        let mut indices = (0..step_dec.len()).collect::<Vec<_>>();
        indices.sort_unstable_by(|&i, &j| step_dec[j].partial_cmp(&step_dec[i]).unwrap());
        indices.truncate(k);

        let rng = &mut rand::rng();
        let idx = indices.choose(rng).unwrap();
        *idx
    }

    fn top_p(&self, step_dec: &ndarray::ArrayView1<f64>, top_p: f64, temperature: f64) -> usize {
        let step_dec = step_dec.exp() / temperature;
        let sum = step_dec.sum();
        let step_dec = step_dec / sum;
        let mut sorted = step_dec.iter().copied().enumerate().collect::<Vec<_>>();
        sorted.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        let mut i = 0;
        let mut cumsum = 0.0;
        while cumsum < top_p {
            cumsum += sorted[i].1;
            i += 1;
        }
        let candidates = &sorted[..i].iter().map(|(i, _)| i).collect::<Vec<_>>();
        let rng = &mut rand::rng();
        let idx = candidates.choose(rng).unwrap();
        **idx
    }

    fn decode(&self, x: &ndarray::ArrayView1<f64>) -> usize {
        match &self.strategy {
            Strategy::Greedy => self.greedy(x),
            Strategy::TopK(StrategyTopK { k }) => self.top_k(x, *k),
            Strategy::TopP(StrategyTopP { top_p, temperature }) => {
                self.top_p(x, *top_p, *temperature)
            }
        }
    }

    fn forward(&self, source: &ndarray::Array1<usize>) -> ndarray::Array1<usize> {
        let e_emb = self.e_emb.forward(source);
        let (enc_out, _) = self.encoder.forward(&e_emb.view(), None);
        let (enc_out_rev, _) = self.encoder_reverse.forward(&e_emb.view(), None);
        let enc_out = ndarray::concatenate(
            ndarray::Axis(enc_out.ndim() - 1),
            &[enc_out.view(), enc_out_rev.view()],
        )
        .unwrap();
        let enc_out = self.encoder_fc.forward_2d(&enc_out.view());
        let enc_out = enc_out.mapv(|x| x.tanh());
        let mut result = vec![constants::SOS_IDX];
        let mut h1: Option<ndarray::Array1<f64>> = None;
        let mut h2: Option<ndarray::Array1<f64>> = None;
        for _ in 0..self.max_len {
            let dec_emb = self
                .k_emb
                .forward(&ndarray::Array1::from_elem(1, *result.last().unwrap()));
            let (dec_out, h1_) = match h1 {
                Some(h1) => self.pre_decoder.forward(&dec_emb.view(), Some(h1.view())),
                None => self.pre_decoder.forward(&dec_emb.view(), None),
            };
            h1 = Some(h1_);
            let attn_out = self
                .attn
                .forward(&dec_out.view(), &enc_out.view(), &enc_out.view());
            let x = ndarray::concatenate(
                ndarray::Axis(dec_out.ndim() - 1),
                &[dec_out.view(), attn_out.view()],
            )
            .unwrap();
            let (x, h2_) = match h2 {
                Some(h2) => self.post_decoder.forward(&x.view(), Some(h2.view())),
                None => self.post_decoder.forward(&x.view(), None),
            };
            h2 = Some(h2_);
            let x = self.fc.forward_2d(&x.view());
            let x = x.index_axis(ndarray::Axis(0), 0);
            result.push(self.decode(&x));
            if result.last().unwrap() == &constants::EOS_IDX {
                break;
            }
        }

        ndarray::Array1::from(result)
    }
}
