[
    Diagnostic {
        range: Range {
            start: Position {
                line: 13,
                character: 5,
            },
            end: Position {
                line: 13,
                character: 11,
            },
        },
        severity: Some(
            Error,
        ),
        code: None,
        source: None,
        message: "expected type after `:`",
        related_information: Some(
            [
                DiagnosticRelatedInformation {
                    location: Location {
                        uri: Url {
                            scheme: "file",
                            cannot_be_a_base: false,
                            username: "",
                            password: None,
                            host: None,
                            port: None,
                            path: "(local-file-prefix)/dada_tests/parser/param_class.dada",
                            query: None,
                            fragment: None,
                        },
                        range: Range {
                            start: Position {
                                line: 12,
                                character: 5,
                            },
                            end: Position {
                                line: 13,
                                character: 5,
                            },
                        },
                    },
                    message: "`:` is here",
                },
            ],
        ),
        tags: None,
    },
]