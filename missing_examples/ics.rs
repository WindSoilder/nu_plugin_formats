fn examples(&self) -> Vec<Example> {
    vec![Example {
        example: "'BEGIN:VCALENDAR
END:VCALENDAR' | from ics",
        description: "Converts ics formatted string to table",
        result: Some(Value::List {
            vals: vec![Value::Record {
                cols: vec![
                    "properties".to_string(),
                    "events".to_string(),
                    "alarms".to_string(),
                    "to-Dos".to_string(),
                    "journals".to_string(),
                    "free-busys".to_string(),
                    "timezones".to_string(),
                ],
                vals: vec![
                    Value::List {
                        vals: vec![],
                        span: Span::test_data(),
                    },
                    Value::List {
                        vals: vec![],
                        span: Span::test_data(),
                    },
                    Value::List {
                        vals: vec![],
                        span: Span::test_data(),
                    },
                    Value::List {
                        vals: vec![],
                        span: Span::test_data(),
                    },
                    Value::List {
                        vals: vec![],
                        span: Span::test_data(),
                    },
                    Value::List {
                        vals: vec![],
                        span: Span::test_data(),
                    },
                    Value::List {
                        vals: vec![],
                        span: Span::test_data(),
                    },
                ],
                span: Span::test_data(),
            }],
            span: Span::test_data(),
        }),
    }]
}
