use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const STRATEGY_TS: &'static str = r#"
/** 推論アルゴリズム */
export type Strategy = {
    type: "greedy";
} | {
    type: "topK";
    k: number;
} | {
    type: "topP";
    topP: number;
    temperature: number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Strategy")]
    pub type JsStrategy;
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum Strategy {
    Greedy,
    #[serde(rename_all = "camelCase")]
    TopK { k: usize },
    #[serde(rename_all = "camelCase")]
    TopP { top_p: f32, temperature: f32 },
}
impl Strategy {
    fn into(self) -> e2k::Strategy {
        match self {
            Strategy::Greedy => e2k::Strategy::Greedy,
            Strategy::TopK { k } => e2k::Strategy::TopK(e2k::StrategyTopK { k }),
            Strategy::TopP { top_p, temperature } => {
                e2k::Strategy::TopP(e2k::StrategyTopP { top_p, temperature })
            }
        }
    }
}

#[wasm_bindgen]
pub struct C2k {
    inner: e2k::C2k,
}

/// 英単語 -> カタカナの変換器。
#[wasm_bindgen]
impl C2k {
    /// 新しいインスタンスを生成する。
    ///
    /// @param {number} maxLen 読みの最大長。
    #[wasm_bindgen(constructor)]
    pub fn new(#[wasm_bindgen(js_name = "maxLen")] max_len: usize) -> Self {
        Self {
            inner: e2k::C2k::new(max_len),
        }
    }

    /// 推論を行う。
    ///
    /// @param {string} src 変換元の文字列
    pub fn infer(&self, src: &str) -> String {
        self.inner.infer(src)
    }

    /// アルゴリズムを設定する。
    ///
    /// @param {Strategy} strategy アルゴリズム
    #[wasm_bindgen(js_name = setDecodeStrategy)]
    pub fn set_decode_strategy(&mut self, strategy: JsStrategy) -> Result<(), String> {
        let strategy: Strategy =
            serde_wasm_bindgen::from_value(strategy.into()).map_err(|e| e.to_string())?;
        self.inner.set_decode_strategy(strategy.into());

        Ok(())
    }
}

/// 発音 -> カタカナの変換器。
#[wasm_bindgen]
pub struct P2k {
    inner: e2k::P2k,
}

#[wasm_bindgen]
impl P2k {
    /// 新しいインスタンスを生成する。
    ///
    /// @param {number} maxLen 読みの最大長。
    #[wasm_bindgen(constructor)]
    pub fn new(#[wasm_bindgen(js_name = "maxLen")] max_len: usize) -> Self {
        Self {
            inner: e2k::P2k::new(max_len),
        }
    }

    /// 推論を行う。
    ///
    /// @param {Array<string>} pronunciation 変換元の発音。CMUdictのフォーマットに従う。
    pub fn infer(&self, pronunciation: Vec<String>) -> String {
        self.inner
            .infer(&pronunciation.iter().map(|s| s.as_str()).collect::<Vec<_>>())
    }

    /// アルゴリズムを設定する。
    ///
    /// @param {Strategy} strategy アルゴリズム
    #[wasm_bindgen(js_name = setDecodeStrategy)]
    pub fn set_decode_strategy(&mut self, strategy: JsStrategy) -> Result<(), String> {
        let strategy: Strategy =
            serde_wasm_bindgen::from_value(strategy.into()).map_err(|e| e.to_string())?;
        self.inner.set_decode_strategy(strategy.into());

        Ok(())
    }
}
