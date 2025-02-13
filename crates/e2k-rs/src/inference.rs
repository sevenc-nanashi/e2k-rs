use crate::{constants, layers};
use educe::Educe;
use std::{
    collections::HashMap,
    io::{Read, Seek},
};

#[derive(Debug)]
pub enum Strategy {
    Greedy,
    TopK(StrategyTopK),
    TopP { top_p: f32, temperature: f32 },
}
#[derive(Debug, Educe)]
#[educe(Default)]
pub struct StrategyTopK {
    #[educe(Default(expression = 3))]
    k: usize,
}

#[derive(Debug, Educe)]
#[educe(Default)]
pub struct TopP {
    #[educe(Default(expression = 0.9))]
    top_p: f32,
    #[educe(Default(expression = 1.0))]
    temperature: f32,
}


/// 英単語 -> カタカナ。
pub struct P2k {
    inner: E2k,
}

struct E2k {
    s2s: S2s,
    in_table: HashMap<char, usize>,
    out_table: HashMap<usize, char>,
}

impl E2k {
    fn infer(&self, input: &str) -> String {
        let source = [constants::SOS_IDX]
            .into_iter()
            .chain(input.chars().filter_map(|c| self.in_table.get(&c).copied()))
            .chain([constants::SOS_IDX].into_iter());
        let source = ndarray::Array1::from_iter(source);
        let target = self.s2s.forward(&source);
        todo!()
    }
}

struct S2s {
    e_emb: layers::Embedding<f32>,
    k_emb: layers::Embedding<f32>,
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
    fn new<R: Read + Seek>(weights: ndarray_npz::NpzReader<R>) -> Self {
        todo!()
    }
    fn forward(&self, source: &ndarray::Array1<usize>) -> ndarray::Array1<usize> {
        todo!()
    }
}
