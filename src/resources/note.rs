//! ノーツはyamlで書けるように構造体を作る.
//! またすべてのyamlパース用構造体にはDeserializeとSerializeを実装する.
//! こうすることで実際の譜面データからyamlとその逆の変換が可能であることを保証する.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NoteSpawnParser {
    note: NoteTypeParser,
    /// 小節番号（0始まり）
    bar: u32,
    /// 小節内の拍位置（0始まり）. 例えば1.5なら2拍目の裏になる
    beat: f64,
}

/// YAMLファイルのノーツ情報パース用構造体
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum NoteTypeParser {
    Normal {
        key: i32,
    },
    BarLine,
    AdLib {
        key: i32,
    },
    /// lenは拍数で指定
    Long {
        key: i32,
        len: f32,
        /// 同一のノートであることを確かめられるように, イベント送信時に適当な数字を挿入する.
        id: u32,
    },
}

#[derive(Debug, Clone)]
pub struct NoteSpawn {
    pub note_type: NoteType,
    pub bar: u32,
    pub beat: f64,
}
impl From<NoteSpawnParser> for NoteSpawn {
    fn from(data: NoteSpawnParser) -> Self {
        Self {
            note_type: data.note.into(),
            bar: data.bar,
            beat: data.beat,
        }
    }
}
impl From<NoteSpawn> for NoteSpawnParser {
    fn from(data: NoteSpawn) -> Self {
        Self {
            note: data.note_type.into(),
            bar: data.bar,
            beat: data.beat,
        }
    }
}

/// ハッシュ化できない情報を持つノーツや取得できないもあるので, 辞書のキーにするための構造体を作る
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NoteTypeKey {
    Normal {
        key: i32,
    },
    AdLib {
        key: i32,
    },
    Long {
        key: i32,
    },
    /// 集計しないための列挙子
    Other,
}
impl From<NoteType> for NoteTypeKey {
    fn from(ty: NoteType) -> Self {
        Self::from(&ty)
    }
}
impl From<&NoteType> for NoteTypeKey {
    fn from(ty: &NoteType) -> Self {
        match ty {
            NoteType::Normal { key } => NoteTypeKey::Normal { key: *key },
            NoteType::AdLib { key } => NoteTypeKey::AdLib { key: *key },
            NoteType::Long {
                key,
                length: _,
                id: _,
            } => NoteTypeKey::Long { key: *key },
            _ => NoteTypeKey::Other,
        }
    }
}

/// ノーツの種類ごとの情報を保持する構造体.
#[derive(Debug, Clone)]
pub enum NoteType {
    Normal {
        key: i32,
    },
    BarLine,
    /// アドリブノーツ. 見えないためにプレイヤーの自由にリズムを取れる. 逃してもミスにならない.
    AdLib {
        key: i32,
    },
    Long {
        key: i32,
        length: f32,
        id: u32,
    },
}
impl From<NoteTypeParser> for NoteType {
    fn from(data: NoteTypeParser) -> Self {
        match data {
            NoteTypeParser::Normal { key } => NoteType::Normal { key },
            NoteTypeParser::BarLine => NoteType::BarLine,
            NoteTypeParser::AdLib { key } => NoteType::AdLib { key },
            NoteTypeParser::Long { key, len, id } => NoteType::Long {
                key,
                length: len,
                id,
            },
        }
    }
}
impl From<NoteType> for NoteTypeParser {
    fn from(data: NoteType) -> Self {
        match data {
            NoteType::Normal { key } => NoteTypeParser::Normal { key },
            NoteType::BarLine => NoteTypeParser::BarLine,
            NoteType::AdLib { key } => NoteTypeParser::AdLib { key },
            NoteType::Long { key, length, id } => NoteTypeParser::Long {
                key,
                len: length,
                id,
            },
        }
    }
}

#[test]
fn yaml_test() {
    let y = vec![
        NoteSpawnParser {
            bar: 0,
            beat: 0.0,
            note: NoteTypeParser::Normal { key: 0 },
        },
        NoteSpawnParser {
            bar: 0,
            beat: 0.5,
            note: NoteTypeParser::Normal { key: 1 },
        },
    ];
    println!("{}", serde_yaml::to_string(&y).unwrap());

    let s = "- { bar: 0, beat: 0.0, note: !Normal { key: 0 } }\n- { bar: 0, beat: 0.5, note: !Normal { key: 1 } }";

    println!("{:?}", serde_yaml::from_str::<Vec<NoteSpawnParser>>(s));
}
