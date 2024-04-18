use deno_core::{error, op2, OpState, Resource, serde_json};

/// 返回值
#[derive(Debug)]
pub struct ReturnValue {
    value: serde_json::Value,
}

impl ReturnValue {
    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }
}

/// 返回值实现资源名称特征
impl Resource for ReturnValue {
    fn name(&self) -> std::borrow::Cow<str> {
        "ReturnValue".into()
    }
}

/// 任意错误
pub type AnyError = error::AnyError;
/// 权限
pub type Permissions = deno_runtime::permissions::Permissions;

/// 返回值操作<br/>
/// state: 状态<br/>
/// args: 返回值
#[op2]
fn op_return(state: &mut OpState, #[serde] args: serde_json::Value) {
    state.resource_table.add(ReturnValue { value: args });
}

deno_core::extension!(
    script_runtime,
    ops = [op_return],
    esm_entry_point = "ext:script_runtime/tests/runtime.js",
    esm = ["tests/runtime.js"],
);