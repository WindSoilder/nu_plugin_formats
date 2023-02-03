fn examples(&self) -> Vec<Example> {
    vec![Example {
        example: "'BEGIN:VCARD
N:Foo
FN:Bar
EMAIL:foo@bar.com
END:VCARD' | from vcf",
        description: "Converts ics formatted string to table",
        result: Some(Value::List {
            vals: vec![Value::Record {
                cols: vec!["properties".to_string()],
                vals: vec![Value::List {
                    vals: vec![
                        Value::Record {
                            cols: vec![
                                "name".to_string(),
                                "value".to_string(),
                                "params".to_string(),
                            ],
                            vals: vec![
                                Value::test_string("N"),
                                Value::test_string("Foo"),
                                Value::Nothing {
                                    span: Span::test_data(),
                                },
                            ],
                            span: Span::test_data(),
                        },
                        Value::Record {
                            cols: vec![
                                "name".to_string(),
                                "value".to_string(),
                                "params".to_string(),
                            ],
                            vals: vec![
                                Value::test_string("FN"),
                                Value::test_string("Bar"),
                                Value::Nothing {
                                    span: Span::test_data(),
                                },
                            ],
                            span: Span::test_data(),
                        },
                        Value::Record {
                            cols: vec![
                                "name".to_string(),
                                "value".to_string(),
                                "params".to_string(),
                            ],
                            vals: vec![
                                Value::test_string("EMAIL"),
                                Value::test_string("foo@bar.com"),
                                Value::Nothing {
                                    span: Span::test_data(),
                                },
                            ],
                            span: Span::test_data(),
                        },
                    ],
                    span: Span::test_data(),
                }],
                span: Span::test_data(),
            }],
            span: Span::test_data(),
        }),
    }]
}
