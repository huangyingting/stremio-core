use crate::types::profile::Settings;
use serde_test::{assert_tokens, Token};
use url::Url;

#[test]
fn settings() {
    assert_tokens(
        &Settings {
            interface_language: "interface_language".to_owned(),
            streaming_server_url: Url::parse("https://streaming_server_url").unwrap(),
            binge_watching: true,
            play_in_background: true,
            play_in_external_player: true,
            hardware_decoding: true,
            subtitles_language: "subtitles_language".to_owned(),
            subtitles_size: 1,
            subtitles_font: "subtitles_font".to_owned(),
            subtitles_bold: true,
            subtitles_offset: 1,
            subtitles_text_color: "subtitles_text_color".to_owned(),
            subtitles_background_color: "subtitles_background_color".to_owned(),
            subtitles_outline_color: "subtitles_outline_color".to_owned(),
        },
        &[
            Token::Struct {
                name: "Settings",
                len: 14,
            },
            Token::Str("interfaceLanguage"),
            Token::Str("interface_language"),
            Token::Str("streamingServerUrl"),
            Token::Str("https://streaming_server_url/"),
            Token::Str("bingeWatching"),
            Token::Bool(true),
            Token::Str("playInBackground"),
            Token::Bool(true),
            Token::Str("playInExternalPlayer"),
            Token::Bool(true),
            Token::Str("hardwareDecoding"),
            Token::Bool(true),
            Token::Str("subtitlesLanguage"),
            Token::Str("subtitles_language"),
            Token::Str("subtitlesSize"),
            Token::U8(1),
            Token::Str("subtitlesFont"),
            Token::Str("subtitles_font"),
            Token::Str("subtitlesBold"),
            Token::Bool(true),
            Token::Str("subtitlesOffset"),
            Token::U8(1),
            Token::Str("subtitlesTextColor"),
            Token::Str("subtitles_text_color"),
            Token::Str("subtitlesBackgroundColor"),
            Token::Str("subtitles_background_color"),
            Token::Str("subtitlesOutlineColor"),
            Token::Str("subtitles_outline_color"),
            Token::StructEnd,
        ],
    );
}
