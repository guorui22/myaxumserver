use std::path::Path;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use deno_core::{FastString, FsModuleLoader, ModuleSpecifier, PollEventLoopOptions, serde_json, url};
use deno_core::error::AnyError;
use deno_runtime::BootstrapOptions;
use deno_runtime::permissions::{Permissions, PermissionsContainer};
use deno_runtime::worker::{MainWorker, WorkerOptions};

use libjsandbox::script;
use libjsandbox::script::script_runtime;

#[tokio::test]
async fn call_06() -> Result<(), AnyError> {

    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/runtime.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        PermissionsContainer::new(Permissions::allow_all()),
        WorkerOptions {
            module_loader: Rc::new(FsModuleLoader),
            extensions: vec![script_runtime::init_ops()],
            bootstrap: BootstrapOptions {
                enable_testing_features: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let _r1 = worker.execute_script("", FastString::from_static(include_str!("output_01.js")))?;

    let a = serde_json::json!({"a1":1000, "a2": 2000}).to_string();

    let timeout = Duration::from_secs(3);
    if timeout > Duration::ZERO {
        let handle = worker.js_runtime.v8_isolate().thread_safe_handle();
        thread::spawn(move || {
            thread::sleep(timeout);
            handle.terminate_execution();
        });
    }

    let script_code = r#"
    var op_return;
    import("ext:core/ops").then((imported) => {{
        op_return = imported.op_return;
        console.log(imported);
    }})
    "#.to_string();
    worker.execute_script("ext:<anon>", script_code.into())?;
    worker.js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;

    let func = "output_01.for_in_object";
    worker.execute_script(
        "",
        format!(
            r#"
                (async () => {{
                    console.log(Deno[Deno.internal].core.ops);
                    console.log('===============================');
                    console.log('===============================');
                    op_return(
                        {func}.constructor.name === 'AsyncFunction' ? await {func}({a}) : {func}({a})
                    );
                }})();
            "#
        ).into(),
    )?;

    worker.js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;

    let state_rc = worker.js_runtime.op_state();
    let mut state = state_rc.borrow_mut();
    // let result: std::rc::Rc<ReturnValue>;
    let mut rid: u32 = 0;
    for val in state.resource_table.names() {
        dbg!(&val.1);
        if val.1 == "ReturnValue" {
            rid = val.0;
        }
    }

    dbg!(rid);

    for x in state.resource_table.names() {
        dbg!(&x.0, &x.1);
    }

    // let result = state.resource_table.take::<jsandbox::script::ReturnValue>(rid).unwrap();
    let result = state.resource_table.take::<script::ReturnValue>(rid).unwrap();

    dbg!(&result.value().as_array());

    assert_eq!(789, 789);
    Ok(())
}
