#[cfg(test)]
mod test {
    use core::str::FromStr;

    use edn_rs::Edn;

    fn debug_msg(s: &str) -> String {
        let err = Edn::from_str(s).err().unwrap();
        format!("{err:?}")
    }

    #[test]
    fn invalid_keyword() {
        assert_eq!(
            debug_msg(":"),
            "EdnError { code: InvalidKeyword, line: Some(1), column: Some(1) }"
        );
        assert_eq!(
            debug_msg("  :"),
            "EdnError { code: InvalidKeyword, line: Some(1), column: Some(3) }"
        );
        assert_eq!(
            debug_msg("\n\n   :"),
            "EdnError { code: InvalidKeyword, line: Some(3), column: Some(4) }"
        );
    }

    #[test]
    fn unexpected_eof() {
        assert_eq!(
            debug_msg(r#""hello, world!"#),
            "EdnError { code: UnexpectedEOF, line: Some(1), column: Some(15) }"
        );
        assert_eq!(
            debug_msg(
                r#""hello,
multiple
lines
world!"#
            ),
            "EdnError { code: UnexpectedEOF, line: Some(4), column: Some(7) }"
        );
    }

    #[test]
    #[cfg(not(feature = "sets"))]
    fn disabled_features() {
        // Special case of running into a set without the feature enabled
        assert_eq!(
            debug_msg("#{true, \\c, 3,four, }",),
            "EdnError { code: NoFeatureSets, line: Some(1), column: Some(2) }"
        );
    }
}
