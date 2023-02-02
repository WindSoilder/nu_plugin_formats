use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, Signature, SyntaxShape, Value};

pub struct FromEml;

impl Plugin for FromEml {
    fn signature(&self) -> Vec<Signature> {
        unimplemented!()
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        unimplemented!()
    }
}
