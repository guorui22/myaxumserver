use deno_core::{error, op2, OpState, Resource, serde_json};

/// 返回值
#[derive(Debug)]
pub struct ReturnValue {
    value: serde_json::Value,
}
/// 返回值引用
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

/// 保存返回值到资源表<br/>
/// state: 状态<br/>
/// args: 返回值
#[op2]
fn op_return(state: &mut OpState, #[serde] args: serde_json::Value) {
    state.resource_table.add(ReturnValue { value: args });
}

// 扩展 op_return 函数到 script_runtime 模块
deno_core::extension!(
    script_runtime,
    ops = [op_return],
    esm_entry_point = "ext:script_runtime/src/runtime.js",
    esm = ["src/runtime.js"],
);