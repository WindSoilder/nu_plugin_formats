mod from;

use from::{eml, ics};
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, Signature, SyntaxShape, Type, Value};

pub struct FromCmds;

impl Plugin for FromCmds {
    fn signature(&self) -> Vec<Signature> {
        vec![
            Signature::build(eml::CMD_NAME)
                .input_output_types(vec![(Type::String, Type::Record(vec![]))])
                .named(
                    "preview-body",
                    SyntaxShape::Int,
                    "How many bytes of the body to preview",
                    Some('b'),
                )
                .category(Category::Formats),
            Signature::build(ics::CMD_NAME)
                .input_output_types(vec![(Type::String, Type::Table(vec![]))])
                .category(Category::Formats),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            eml::CMD_NAME => eml::from_eml_call(call, input),
            ics::CMD_NAME => ics::from_ics_call(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
