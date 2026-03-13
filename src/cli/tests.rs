use super::{
    parser::*,
    args::*,
};

#[test]
fn parses_long_help_flag() {
    let args = vec!["--help".to_string()];
    
    let mut parser = CliParser::new();
    parser.read_args(args).unwrap();

    assert!(parser.flags.contains(&Flag::Help));
}

#[test]
fn parses_short_flag() {
    let args = vec!["-h".to_string()];
    
    let mut parser = CliParser::new();
    parser.read_args(args).unwrap();

    assert!(parser.flags.contains(&Flag::Help));
}

#[test]
fn parses_short_flag_sequence() {
    let args = vec!["-hvo".to_string(), "/this/is/some/path".to_string()];
    
    let mut parser = CliParser::new();
    parser.read_args(args).unwrap();

    assert!(parser.flags.contains(&Flag::Help));
    assert!(parser.flags.contains(&Flag::Verbose { level: 1 }));
    assert!(parser.flags.contains(&Flag::Output { path: "/this/is/some/path".to_string() }));
}
