use crate::{
    command::{
        Command,
        IfRhs,
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
