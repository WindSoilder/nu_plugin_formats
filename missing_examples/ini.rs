fn examples(&self) -> Vec<Example> {
    vec![Example {
        example: "'[foo]
a=1
b=2' | from ini",
        description: "Converts ini formatted string to record",
        result: Some(Value::Record {
            cols: vec!["foo".to_string()],
            vals: vec![Value::Record {
                cols: vec!["a".to_string(), "b".to_string()],
                vals: vec![Value::test_string("1"), Value::test_string("2")],
                span: Span::test_data(),
            }],
            span: Span::test_data(),
        }),
    }]
}
