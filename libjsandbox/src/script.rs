use std::thread;
use std::time::Duration;
use deno_core::{error, FastString, op2, OpState, PollEventLoopOptions, Resource, serde, serde_json, url};
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::{MainWorker, WorkerOptions};
use crate::args::Args;

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

/// 脚本<br/>
/// code: 初始化 JS 代码<br/>
/// timeout: 脚本执行超时设置<br/>
/// permissions: 权限设置<br/>
/// worker: 工作线程
pub struct Script {
    timeout: Duration,
    permissions: Permissions,
    worker: Option<MainWorker>,
}


impl Script {

    /// 设置超时<br/>
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 设置权限<br/>
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// 构建脚本实例<br/>
    pub fn build() -> Result<Self, error::AnyError> {

        Ok(Script {
            timeout: Duration::ZERO,
            permissions: Permissions::none_with_prompt(),
            worker: Some(MainWorker::bootstrap_from_options(
                url::Url::parse("data:text/plain,").unwrap(),
                PermissionsContainer::new(deno_runtime::deno_permissions::Permissions::allow_all()),
                WorkerOptions {
                    extensions: vec![script_runtime::init_ops_and_esm()],
                    ..Default::default()
                },
            )),
        })

    }

    pub fn add_script(&mut self, code: &'static str) -> Result<(), error::AnyError> {
        // 获取工作线程可变引用
        let worker = self.worker.as_mut().unwrap();
        // 执行脚本以导入自定义函数
        worker.execute_script("", FastString::from_static(code))?;
        // 返回脚本实例
        Ok(())
    }

    /// 调用脚本实例中的函数<br/>
    /// func: 函数名<br/>
    /// args: 函数参数
    pub async fn call<T, R>(&mut self, func: &str, args: T) -> Result<R, error::AnyError>
        where
            T: Args,
            R: serde::de::DeserializeOwned,
    {
        // 检查工作线程是否初始化
        if self.worker.is_none() {
            return Err(error::AnyError::msg(
                "Script 实例没有被正确初始化, 是不是没有执行 build() 方法?",
            ));
        }

        // 获取工作线程可变引用
        let worker = self.worker.as_mut().unwrap();
        // 转换输入参数为字符串
        let input = args.to_string()?;

        // 设置脚本执行超时时间
        if self.timeout > Duration::ZERO {
            let handle = worker.js_runtime.v8_isolate().thread_safe_handle();
            let timeout = self.timeout;
            thread::spawn(move || {
                thread::sleep(timeout);
                handle.terminate_execution();
            });
        }

        // 调用脚本中的函数
        worker.execute_script(
            "",
            format!(
                "
                (async () => {{
                    globalThis.myOps.op_return(
                        {func}.constructor.name === 'AsyncFunction' ? await {func}({input}) : {func}({input})
                    );
                }})();
            "
            ).into(),
        )?;

        // 等待事件循环结束
        worker.js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;

        // 获取上述函数返回值
        let state_rc = worker.js_runtime.op_state();
        let mut state = state_rc.borrow_mut();
        let mut rid: u32 = 0;
        for val in state.resource_table.names() {
            if val.1 == "ReturnValue" {
                rid = val.0;
            }
        }
        let result = state.resource_table.take::<ReturnValue>(rid).unwrap();

        Ok(serde_json::from_value(result.value.clone()).unwrap())
    }
}

