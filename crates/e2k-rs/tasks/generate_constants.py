import os
from e2k import constants


destination = os.path.join(os.path.dirname(__file__), "../src/constants.rs")
with open(destination, "w") as f:
    f.write(
        f"""
// `task generate:constants` により生成。
// このファイルは直接編集しないでください。

pub const PAD_IDX: usize = {constants.PAD_IDX};
pub const SOS_IDX: usize = {constants.SOS_IDX};
pub const EOS_IDX: usize = {constants.EOS_IDX};
pub const EN_PHONES: [&str; {len(constants.en_phones)}] = [
    {", ".join([f'"{phone}"' for phone in constants.en_phones])}
];
pub const KANAS: [&str; {len(constants.kanas)}] = [
    {", ".join([f'"{kana}"' for kana in constants.kanas])}
];
pub const ASCII_ENTRIES: [&str; {len(constants.ascii_entries)}] = [
    {", ".join([f'"{entry}"' for entry in constants.ascii_entries])}
];
"""
    )
