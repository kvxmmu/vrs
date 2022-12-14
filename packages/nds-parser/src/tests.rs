use crate::{
    command::{
        ChoiceOption,
        ClearTextType,
        Command,
        IfRhs,
        MusicFile,
        SoundLooping,
        VariableModifier,
        VariableStorageType,
    },
    prelude::ParseScript,
    text::{
        Foreground,
        Text,
        TextSpan,
        TextType,
    },
};

#[test]
fn test_cleartext() {
    let commands = "cleartext\ncleartext !".parse_script().unwrap();

    assert_eq!(
        commands,
        [
            Command::ClearText(ClearTextType::FillBottomScreen),
            Command::ClearText(ClearTextType::TextBufferInclHistory)
        ]
    );
}

#[test]
fn test_random() {
    let commands = "random var 0 10".parse_script().unwrap();

    assert_eq!(
        commands,
        [Command::Random {
            variable: "var".into(),
            range: 0..=10
        }]
    );
}

#[test]
fn test_delay() {
    let commands = "delay 100".parse_script().unwrap();

    assert_eq!(commands, [Command::Delay { frames: 100 }]);
}

#[test]
fn test_jump() {
    let commands = "jump file.scr\njump fuck.scr label"
        .parse_script()
        .unwrap();

    assert_eq!(
        commands,
        [
            Command::Jump {
                file: "file.scr".into(),
                label: None
            },
            Command::Jump {
                file: "fuck.scr".into(),
                label: Some("label".into())
            }
        ]
    );
}

#[test]
fn test_label_and_goto() {
    let commands = "label test\ngoto test".parse_script().unwrap();
    assert_eq!(
        commands,
        [
            Command::Label("test".to_owned()),
            Command::Goto("test".to_owned())
        ]
    );
}

#[test]
fn test_choice() {
    let commands = "choice hello|world|$name".parse_script().unwrap();

    assert_eq!(
        commands,
        [Command::Choice {
            options: vec![
                ChoiceOption::Option("hello".to_owned()),
                ChoiceOption::Option("world".to_owned()),
                ChoiceOption::Variable("name".to_owned()),
            ]
        }]
    );
}

#[test]
fn test_music() {
    let commands = "music bgm.mp3\nmusic ~".parse_script().unwrap();

    assert_eq!(
        commands,
        [
            Command::Music {
                file: MusicFile::Path("bgm.mp3".into())
            },
            Command::Music {
                file: MusicFile::StopPlaying
            }
        ]
    );
}

#[test]
fn test_sound() {
    let commands = "sound bg0.aac -1\nsound ~\nsound waves.aac -1"
        .parse_script()
        .unwrap();
    assert_eq!(
        commands,
        [
            Command::Sound(SoundLooping::Infinite {
                file: "bg0.aac".into()
            }),
            Command::Sound(SoundLooping::StopCurrentlyPlaying),
            Command::Sound(SoundLooping::Infinite {
                file: "waves.aac".into()
            })
        ]
    );
}

#[test]
fn test_setvar() {
    let commands = "setvar affection = 10\ngsetvar progress = 0"
        .parse_script()
        .unwrap();

    assert_eq!(
        commands,
        [
            Command::SetVar {
                name: "affection".to_owned(),
                accumulator: 10,
                modifier: VariableModifier::Assign,
                storage: VariableStorageType::Local
            },
            Command::SetVar {
                name: "progress".to_owned(),
                accumulator: 0,
                modifier: VariableModifier::Assign,
                storage: VariableStorageType::Global
            },
        ]
    );
}

#[test]
fn test_setimg() {
    let commands = "setimg test.jpg 10 10".parse_script().unwrap();

    assert_eq!(
        commands,
        [Command::SetImg {
            file: "test.jpg".into(),
            coordinates: (10, 10)
        }]
    );
}

#[test]
fn test_bgload() {
    let commands = "bgload test.jpg 20\nbgload test.jpg"
        .parse_script()
        .unwrap();

    assert_eq!(
        commands,
        [
            Command::BgLoad {
                file: "test.jpg".into(),
                fadetime: 20
            },
            Command::BgLoad {
                file: "test.jpg".into(),
                fadetime: 16
            }
        ]
    );
}

#[test]
fn test_branch() {
    let commands = "if a == b\nfi\nif a == 10\nfi"
        .parse_script()
        .unwrap();

    assert_eq!(
        commands,
        [
            Command::If {
                name: "a".to_owned(),
                rhs: IfRhs::Variable("b".to_owned())
            },
            Command::EndIf,
            Command::If {
                name: "a".to_owned(),
                rhs: IfRhs::Number(10)
            },
            Command::EndIf,
        ]
    );
}

#[test]
fn test_text() {
    let commands = "text hello $world real world \\x1b[30;1m{$name}-chan"
        .parse_script()
        .unwrap();

    assert_eq!(
        commands,
        [Command::Text(Text {
            spans: vec![
                TextSpan {
                    text: TextType::Plain("hello ".to_owned()),
                    color: Foreground::Regular,
                },
                TextSpan {
                    text: TextType::Variable("world".to_owned()),
                    color: Foreground::Regular,
                },
                TextSpan {
                    text: TextType::Plain("real world ".to_owned()),
                    color: Foreground::Regular,
                },
                TextSpan {
                    text: TextType::Variable("name".to_owned()),
                    color: Foreground::Black,
                },
                TextSpan {
                    text: TextType::Plain("-chan".to_owned()),
                    color: Foreground::Black,
                },
            ]
        })]
    );
}
