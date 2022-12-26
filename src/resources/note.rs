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
    Normal { key: i32 },
    BarLine,
    AdLib { key: i32 },
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

/// ノーツの種類ごとの情報を保持する構造体.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum NoteType {
    Normal {
        key: i32,
    },
    BarLine,
    /// アドリブノーツ. 見えないためにプレイヤーの自由にリズムを取れる. 逃してもミスにならない.
    AdLib {
        key: i32,
    },
}
impl From<NoteTypeParser> for NoteType {
    fn from(data: NoteTypeParser) -> Self {
        match data {
            NoteTypeParser::Normal { key } => NoteType::Normal { key },
            NoteTypeParser::BarLine => NoteType::BarLine,
            NoteTypeParser::AdLib { key } => NoteType::AdLib { key },
        }
    }
}
impl From<NoteType> for NoteTypeParser {
    fn from(data: NoteType) -> Self {
        match data {
            NoteType::Normal { key } => NoteTypeParser::Normal { key },
            NoteType::BarLine => NoteTypeParser::BarLine,
            NoteType::AdLib { key } => NoteTypeParser::AdLib { key },
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
