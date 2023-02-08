use indexmap::map::IndexMap;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{ShellError, Value};

pub const CMD_NAME: &str = "from ini";

pub fn from_ini_call(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = input.span().unwrap_or(call.head);
    let input_string = input.as_string()?;
    let head = call.head;

    let v: Result<IndexMap<String, IndexMap<String, String>>, serde_ini::de::Error> =
        serde_ini::from_str(&input_string);
    match v {
        Ok(index_map) => {
            let (cols, vals) = index_map
                .into_iter()
                .fold((vec![], vec![]), |mut acc, (k, v)| {
                    let (cols, vals) = v.into_iter().fold((vec![], vec![]), |mut acc, (k, v)| {
                        acc.0.push(k);
                        acc.1.push(Value::String { val: v, span });
                        acc
                    });
                    acc.0.push(k);
                    acc.1.push(Value::Record { cols, vals, span });
                    acc
                });
            Ok(Value::Record { cols, vals, span })
        }
        Err(err) => Err(ShellError::UnsupportedInput(
            format!("Could not load ini: {err}"),
            "value originates from here".into(),
            head,
            span,
        )
        .into()),
    }
}
