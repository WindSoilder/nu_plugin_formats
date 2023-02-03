    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Convert eml structured data into record",
                example: "'From: test@email.com
Subject: Welcome
To: someone@somewhere.com
Test' | from eml",
                result: Some(Value::Record {
                    cols: vec![
                        "Subject".to_string(),
                        "From".to_string(),
                        "To".to_string(),
                        "Body".to_string(),
                    ],
                    vals: vec![
                        Value::test_string("Welcome"),
                        Value::Record {
                            cols: vec!["Name".to_string(), "Address".to_string()],
                            vals: vec![
                                Value::nothing(Span::test_data()),
                                Value::test_string("test@email.com"),
                            ],
                            span: Span::test_data(),
                        },
                        Value::Record {
                            cols: vec!["Name".to_string(), "Address".to_string()],
                            vals: vec![
                                Value::nothing(Span::test_data()),
                                Value::test_string("someone@somewhere.com"),
                            ],
                            span: Span::test_data(),
                        },
                        Value::test_string("Test"),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Convert eml structured data into record",
                example: "'From: test@email.com
Subject: Welcome
To: someone@somewhere.com
Test' | from eml -b 1",
                result: Some(Value::Record {
                    cols: vec![
                        "Subject".to_string(),
                        "From".to_string(),
                        "To".to_string(),
                        "Body".to_string(),
                    ],
                    vals: vec![
                        Value::test_string("Welcome"),
                        Value::Record {
                            cols: vec!["Name".to_string(), "Address".to_string()],
                            vals: vec![
                                Value::nothing(Span::test_data()),
                                Value::test_string("test@email.com"),
                            ],
                            span: Span::test_data(),
                        },
                        Value::Record {
                            cols: vec!["Name".to_string(), "Address".to_string()],
                            vals: vec![
                                Value::nothing(Span::test_data()),
                                Value::test_string("someone@somewhere.com"),
                            ],
                            span: Span::test_data(),
                        },
                        Value::test_string("T"),
                    ],
                    span: Span::test_data(),
                }),
            },
        ]
    }
}

