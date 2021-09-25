mod mountinfo;

use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Dictionary, ReturnSuccess, ReturnValue, Signature, SyntaxShape, UntaggedValue, Value,
};

struct Df;

impl Plugin for Df {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(
            Signature::build("df")
                .switch(
                    "all", "Show all file systems, including include pseudo, duplicate, inaccessible file systems", Some('a')
                )
                .switch(
                    "local", "Show local file systems only.", Some('l'))
        )
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![ReturnSuccess::value(Value::new(
            UntaggedValue::Row(Dictionary::from(call_info.args.named.unwrap())),
            call_info.name_tag,
        ))])
    }

    fn filter(&mut self, _input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        unimplemented!()
    }
}
fn main() {
    serve_plugin(&mut Df {});
}
