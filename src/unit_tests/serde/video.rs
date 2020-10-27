use crate::types::resource::{SeriesInfo, Video};
use crate::unit_tests::serde::default_token_ext::DefaultTokens;
use chrono::prelude::TimeZone;
use chrono::Utc;
use serde_test::{assert_de_tokens, assert_tokens, Token};

#[test]
fn video() {
    assert_tokens(
        &vec![
            Video {
                id: "id".to_owned(),
                title: "title".to_owned(),
                released: Some(Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0)),
                overview: Some("overview".to_owned()),
                thumbnail: Some("thumbnail".to_owned()),
                streams: vec![],
                series_info: Some(SeriesInfo::default()),
                trailer_streams: vec![],
            },
            Video {
                id: "id".to_owned(),
                title: "title".to_owned(),
                released: None,
                overview: None,
                thumbnail: None,
                streams: vec![],
                series_info: None,
                trailer_streams: vec![],
            },
        ],
        &[
            vec![
                Token::Seq { len: Some(2) },
                Token::Map { len: None },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("title"),
                Token::Str("title"),
                Token::Str("released"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00Z"),
                Token::Str("overview"),
                Token::Some,
                Token::Str("overview"),
                Token::Str("thumbnail"),
                Token::Some,
                Token::Str("thumbnail"),
                Token::Str("streams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
            ],
            SeriesInfo::default_token(),
            vec![
                Token::Str("trailerStreams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::MapEnd,
                Token::Map { len: None },
                Token::Str("id"),
                Token::Str("id"),
                Token::Str("title"),
                Token::Str("title"),
                Token::Str("released"),
                Token::None,
                Token::Str("overview"),
                Token::None,
                Token::Str("thumbnail"),
                Token::None,
                Token::Str("streams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("trailerStreams"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::MapEnd,
                Token::SeqEnd,
            ],
        ]
        .concat(),
    );
    assert_de_tokens(
        &Video {
            id: "id".to_owned(),
            title: "".to_owned(),
            released: None,
            overview: None,
            thumbnail: None,
            streams: vec![],
            series_info: None,
            trailer_streams: vec![],
        },
        &[
            Token::Map { len: None },
            Token::Str("id"),
            Token::Str("id"),
            Token::MapEnd,
        ],
    );
}
