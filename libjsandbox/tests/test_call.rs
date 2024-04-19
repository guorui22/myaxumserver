use std::ops::Sub;
use std::thread;
use std::time::{Duration, Instant};

use deno_core::{FastString, PollEventLoopOptions, serde_json, url};
use deno_core::error::AnyError;
use deno_runtime::permissions::{Permissions, PermissionsContainer};
use deno_runtime::worker::{MainWorker, WorkerOptions};

use libjsandbox::script;
use libjsandbox::script::{Script, script_runtime};

#[tokio::test]
async fn call_01() -> Result<(), AnyError> {

    let start_time = Instant::now();

    // 创建脚本实例
    let mut script = Script::build()?
        .permissions(Permissions::allow_all())
        .timeout(Duration::from_secs(3));

    for _ in 0..100 {

        // 导入自定义函数
        script.add_script(include_str!("output_01.js"))?;

        // 调用自定义函数
        let result:serde_json::Value = script.call("output_01.for_in_object", (serde_json::json!({"a1":1000, "a2": 2000}), )).await?;

        // 检查函数返回值
        dbg!(&result.to_string());
        assert_eq!(&result.to_string(), "[1000,2000]");
    }

    println!("Execution time: {} ms", (Instant::now().sub(start_time)).as_millis());

    Ok(())
}

#[tokio::test]
async fn call_02() -> Result<(), AnyError> {

    // 初始化工作线程
    let mut worker = MainWorker::bootstrap_from_options(
        url::Url::parse("data:text/plain,").unwrap(),
        PermissionsContainer::new(Permissions::allow_all()),
        WorkerOptions {
            extensions: vec![script_runtime::init_ops_and_esm()],
            ..Default::default()
        },
    );

    // 执行脚本以导入自定义函数
    worker.execute_script("", FastString::from_static(include_str!("output_01.js")))?;

    // 设置脚本执行超时时间
    let timeout = Duration::from_secs(3);
    if timeout > Duration::ZERO {
        let handle = worker.js_runtime.v8_isolate().thread_safe_handle();
        thread::spawn(move || {
            thread::sleep(timeout);
            handle.terminate_execution();
        });
    }

    // 显示导入的内置函数
    // let script_code = r#"
    // import("ext:core/ops").then((imported) => {{
    //     console.log(imported);
    // }})
    // "#.to_string();
    // worker.execute_script("ext:<anon>", script_code.into())?;
    // worker.js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;

    // 调用自定义函数
    let func = "output_01.for_in_object";   // 自定义函数名
    let input = serde_json::json!({"a1":1000, "a2": 2000}).to_string();  // 函数输入参数
    worker.execute_script(
        "",
        format!(
            r#"
                (async () => {{
                    globalThis.myOps.op_return(
                        {func}.constructor.name === 'AsyncFunction' ? await {func}({input}) : {func}({input})
                    );
                }})();
            "#
        ).into(),
    )?;
    // 等待事件循环结束
    worker.js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;

    // 获取返回值
    let state_rc = worker.js_runtime.op_state();
    let mut state = state_rc.borrow_mut();
    let mut rid: u32 = 0;
    for val in state.resource_table.names() {
        if val.1 == "ReturnValue" {
            rid = val.0;
        }
    }
    let result = state.resource_table.take::<script::ReturnValue>(rid).unwrap();

    // 检查返回值
    dbg!(&result.value().to_string());
    assert_eq!(&result.value().to_string(), "[1000,2000]");

    Ok(())
}
