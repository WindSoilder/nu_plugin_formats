use edn_rs::Edn;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{PluginExample, Span, Value};
use std::str::FromStr;

pub const CMD_NAME: &str = "from edn";

pub fn from_edn_call(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let value = input.as_string()?;
    let edn_result = Edn::from_str(&value);
    match edn_result {
        Err(e) => Err(LabeledError {
            label: "failed to convert to edn".to_string(),
            msg: e.to_string(),
            span: Some(call.head),
        }),
        Ok(edn) => Ok(to_value(edn, call.head)),
    }
}

pub fn examples() -> Vec<PluginExample> {
    vec![]
}

fn to_value(edn: Edn, head: Span) -> Value {
    match edn {
        Edn::Bool(val) => Value::boolean(val, head),
        Edn::Int(val) => Value::int(val as i64, head),
        Edn::Char(val) => Value::string(val, head),
        Edn::Str(val) | Edn::Uuid(val) => Value::string(val, head),
        Edn::UInt(val) => Value::int(val as i64, head),
        Edn::Empty | Edn::Nil => Value::nothing(head),
        Edn::Tagged(tag_name, tagged_edn) => {
            let cols = vec![tag_name];
            let vals = vec![to_value(*tagged_edn, head)];
            Value::record(cols, vals, head)
        }
        Edn::Vector(edns) => {
            let mut vals = vec![];
            for one_edn in edns.to_vec() {
                vals.push(to_value(one_edn, head));
            }
            Value::list(vals, head)
        }
        Edn::Set(edns) => {
            let mut vals = vec![];
            for one_edn in edns.to_set() {
                vals.push(to_value(one_edn, head));
            }
            Value::list(vals, head)
        }
        Edn::Map(edns) => {
            let mut cols = vec![];
            let mut vals = vec![];
            for (edn_col, edn_val) in edns.to_map() {
                cols.push(edn_col);
                vals.push(to_value(edn_val, head));
            }
            Value::record(cols, vals, head)
        }
        Edn::List(edns) => {
            let mut vals = vec![];
            for one_edn in edns.to_vec() {
                vals.push(to_value(one_edn, head));
            }
            Value::list(vals, head)
        }
        Edn::Key(key_name) => Value::string(key_name, head),
        Edn::Symbol(symbol_name) => Value::string(symbol_name, head),
        Edn::NamespacedMap(namespace, namespace_info) => {
            let cols = vec![namespace];

            let mut info_cols = vec![];
            let mut info_vals = vec![];
            for (edn_col, edn_val) in namespace_info.to_map() {
                info_cols.push(edn_col);
                info_vals.push(to_value(edn_val, head));
            }
            let vals = vec![Value::record(info_cols, info_vals, head)];
            Value::record(cols, vals, head)
        }
        Edn::Rational(s) => {
            let cols = vec!["rational".to_string()];
            let vals = vec![Value::string(s, head)];
            Value::record(cols, vals, head)
        }
        Edn::Inst(inst) => {
            let cols = vec!["inst".to_string()];
            let vals = vec![Value::string(inst, head)];
            Value::record(cols, vals, head)
        }
        Edn::Double(number) => {
            let n_str = number.to_string();
            Value::float(n_str.parse::<f64>().unwrap(), head)
        }
    }
}
