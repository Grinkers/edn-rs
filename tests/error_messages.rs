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
            "EdnError { code: InvalidKeyword, line: Some(1), column: Some(1), index: Some(0) }"
        );
        assert_eq!(
            debug_msg("  :"),
            "EdnError { code: InvalidKeyword, line: Some(1), column: Some(3), index: Some(2) }"
        );
        assert_eq!(
            debug_msg("\n\n   :"),
            "EdnError { code: InvalidKeyword, line: Some(3), column: Some(4), index: Some(5) }"
        );
    }

    #[test]
    fn unexpected_eof() {
        assert_eq!(
            debug_msg(r#""hello, world!"#),
            "EdnError { code: UnexpectedEOF, line: Some(1), column: Some(15), index: Some(14) }"
        );
        assert_eq!(
            debug_msg(
                r#""hello,
multiple
lines
world!"#
            ),
            "EdnError { code: UnexpectedEOF, line: Some(4), column: Some(7), index: Some(29) }"
        );
    }

    #[test]
    fn invalid_num() {
        assert_eq!(
            debug_msg(" ,,,, , , ,,,, ,\n ,,,,       0xfoobarlol"),
            "EdnError { code: InvalidNumber, line: Some(2), column: Some(13), index: Some(29) }"
        );
        assert_eq!(
            debug_msg("[ ; comment \n-0xfoobarlol 0xsilycat]"),
            "EdnError { code: InvalidNumber, line: Some(2), column: Some(1), index: Some(13) }"
        );
        assert_eq!(
            debug_msg("[ ;;;,,,,\n , , ,,,, ,\n ,,,,   16  -0xfoobarlol 0xsilycat]"),
            "EdnError { code: InvalidNumber, line: Some(3), column: Some(13), index: Some(34) }"
        );
    }

    #[test]
    fn utf8() {
        assert_eq!(
            debug_msg("(猫 ; cat\nおやつ;treats\n      "),
            "EdnError { code: UnexpectedEOF, line: Some(3), column: Some(7), index: Some(34) }"
        );
    }

    #[test]
    #[cfg(not(feature = "sets"))]
    fn disabled_features() {
        // Special case of running into a set without the feature enabled
        assert_eq!(
            debug_msg("#{true, \\c, 3,four, }",),
            "EdnError { code: NoFeatureSets, line: Some(1), column: Some(2), index: Some(1) }"
        );

        assert_eq!(
            debug_msg("[1 \n2 ;3 \n4 #{true, \\c, 3,four, }]",),
            "EdnError { code: NoFeatureSets, line: Some(3), column: Some(4), index: Some(13) }"
        );
    }
}
