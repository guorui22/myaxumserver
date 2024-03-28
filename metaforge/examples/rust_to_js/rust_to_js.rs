// main.rs
use std::rc::Rc;
use std::time::Instant;
use chrono::expect;

use deno_core::{Extension, FastString, JsRuntime, Op, op2, PollEventLoopOptions, v8};
use deno_core::anyhow::Error;
use deno_core::error::AnyError;
use deno_core::v8::Handle;
use libtracing::error;
#[tokio::main]
async fn main() ->Result<(), deno_core::anyhow::Error>{
    let string = call_js_file_03().await?;
    println!("result: {:?}", string);
    Ok(())
}

// 运行一个自动执行的 JS 脚本
pub async fn call_js_file_03() -> Result<String, deno_core::anyhow::Error> {
    // 创建默认配置JS运行时
    let mut runtime = JsRuntime::new(Default::default());
    // 从 JS 脚本加载函数
    let res = runtime.execute_script(
        "output_01",
        include_str!("/mnt/gr02/RustroverProjects/metaforge_js/dist/output_01.js"))?;
    // 执行脚本里的函数，获取执行结果
    let res = runtime.execute_script(
        "output_01",
        r#"
        // output_01 是 output_01.js 里的一个对象，它在编译时就已经确定
        function Foo() {
            this.a = 11;
            this.b = 12;
        }
        Foo.prototype.c = 1005;
        let hw = output_01.for_in_object(new Foo());
        console.log(hw);
        hw
        "#)?;
    // 获取的 JS 执行结果转换为 Rust 字符串
    let str = res.open(runtime.v8_isolate()).to_rust_string_lossy(&mut runtime.handle_scope());
    // 返回结果
    Ok(str)
}


// 运行一个自动执行的 JS 脚本
pub async fn call_js_file_02() -> Result<String, deno_core::anyhow::Error> {
    // 创建默认配置JS运行时
    let mut runtime = JsRuntime::new(Default::default());
    // 从 JS 脚本加载函数
    let res = runtime.execute_script(
        "output_02",
        include_str!("/mnt/gr02/RustroverProjects/metaforge_js/dist/output_04.js"))?;
    // 执行脚本里的函数，获取执行结果
    let res = runtime.execute_script(
        "output_02",
        r#"
        // output_04 是 output_04.js 里的一个对象，它在编译时就已经确定
        let hw = output_04.hello_world();
        console.log(hw);
        hw
        "#)?;
    // 获取的 JS 执行结果转换为 Rust 字符串
    let str = res.open(runtime.v8_isolate()).to_rust_string_lossy(&mut runtime.handle_scope());
    // 返回结果
    Ok(str)
}


// 运行一个自动执行的 JS 脚本
pub async fn call_js_file_01() -> Result<String, deno_core::anyhow::Error> {
    // 创建默认配置JS运行时
    let mut runtime = JsRuntime::new(Default::default());
    // 执行 JS 脚本
    let res = runtime.execute_script(
        "output_03",
        include_str!("/mnt/gr02/RustroverProjects/metaforge_js/dist/output_03.js"))?;
    // 获取执行结果
    let str = res.open(runtime.v8_isolate()).to_rust_string_lossy(&mut runtime.handle_scope());
    // 返回结果
    Ok(str)
}


pub async fn do_has_param_func_02() -> String {
    let runjs_extension = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[op_test_data_in_out::DECL, op_read_file::DECL, op_write_file::DECL, op_remove_file::DECL, op_fetch::DECL]),
        ..Default::default()
    };


    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    js_runtime.execute_script("[runjs:runtime.js]", FastString::from_static(include_str!("./runtime.js"))).expect("error");

    js_runtime.execute_script(
        "output_01",
        include_str!("output_01.js"))
        .expect("error");

    // 方案-01
    // 时间更短，速度更快
    let res = js_runtime.execute_script(
        "output_01",
        r#"

function Foo() {
    this.a = 11;
    this.b = 12;
}

Foo.prototype.c = 1005;

let arr = output_01.for_in_object(new Foo());
console.log(arr);
JSON.stringify(arr)

// let obj = { c: 8, b: [{z:6,y:5,x:4},7], a: 3 };
// console.log(output_02.json_to_string(obj));
// output_02.json_to_string(obj)

        "#)
        .expect("error");

    let str = res.open(js_runtime.v8_isolate())
        .to_rust_string_lossy(&mut js_runtime.handle_scope());
    str
}


pub async fn do_has_param_func_01() -> String {
    let runjs_extension = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[op_test_data_in_out::DECL, op_read_file::DECL, op_write_file::DECL, op_remove_file::DECL, op_fetch::DECL]),
        ..Default::default()
    };


    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    js_runtime.execute_script("[runjs:runtime.js]", FastString::from_static(include_str!("./runtime.js"))).expect("error");

    let ss = js_runtime.execute_script(
        "how-long-till-lunch.esm1",
        include_str!("how-long-till-lunch.umd.js"))
        .expect("error");

    let start_time = Instant::now();
    // 方案-01
    // 时间更短，速度更快
    let res = js_runtime.execute_script(
            "how-long-till-lunch.esm2",
            "howLongUntilLunch(19, 30)")
            .expect("error");

    let str = res.open(js_runtime.v8_isolate())
        .to_rust_string_lossy(&mut js_runtime.handle_scope());
    // str

    // 方案-02
    let start_time = Instant::now();

    // let scope = &mut js_runtime.handle_scope();
    // let ctx = scope.get_current_context();
    // let global = ctx.global(scope);
    // let key = v8::String::new(scope, "howLongUntilLunch")
    //     .unwrap()
    //     .into();
    // let func: v8::Local<'_, v8::Function> =
    //     global.get(scope, key).unwrap().try_into().unwrap();
    // let recv = v8::undefined(scope).into();
    // let local01 = v8::Integer::new(scope, 19).into();
    // let local02 = v8::Integer::new(scope, 31).into();
    // let res: v8::Local<v8::Value> =
    //     func.call(scope, recv, &[local01, local02]).unwrap().try_into().unwrap();
    // let str = res.to_rust_string_lossy(scope);

    // 记录结束时间
    let end_time = Instant::now();

    // 计算时间间隔
    let duration = end_time.duration_since(start_time);

    // 打印执行时间（以纳秒为单位）
    println!("Execution time: {} nanoseconds", duration.as_nanos());
    str
}

async fn run_js_func(file_path: &str) -> Result<(), AnyError> {


    let main_module = deno_core::resolve_path(file_path, &*std::env::current_dir().unwrap())?;

    let runjs_extension = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[op_read_file::DECL, op_write_file::DECL, op_remove_file::DECL, op_fetch::DECL]),
        ..Default::default()
    };


    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    js_runtime.execute_script("[runjs:runtime.js]", FastString::from_static(include_str!("./runtime.js")))?;

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;

    // 执行 JS 函数
    // let global = js_runtime.get_module_namespace(mod_id).unwrap();
    // let scope = &mut js_runtime.handle_scope();

    // let mut scope = &mut js_runtime.handle_scope();
    // let variable_context = scope.get_current_context();
    // let global = variable_context.global(&mut scope);

    // let func_key = v8::String::new(scope, "sum").unwrap();
    // let func = global.get(scope, func_key.into()).unwrap();
    // dbg!(&func);
    // let func = v8::Local::<v8::Function>::try_from(func).unwrap();
    //
    // let a = v8::Integer::new(scope, 5).into();
    // let b = v8::Integer::new(scope, 2).into();
    // let func_res = func.call(scope, global.into(), &[a, b]).unwrap();
    // let func_res = func_res
    //     .to_string(scope)
    //     .unwrap()
    //     .to_rust_string_lossy(scope);
    // println!("Function returned: {}", func_res);

    let rst = result.await?;
    Ok(rst)
}


async fn run_js(file_path: &str) -> Result<(), Error> {
    let main_module = deno_core::resolve_path(file_path, &*std::env::current_dir().unwrap())?;

    let runjs_extension = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[op_test_data_in_out::DECL, op_read_file::DECL, op_write_file::DECL, op_remove_file::DECL, op_fetch::DECL]),
        ..Default::default()
    };


    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    js_runtime.execute_script("[runjs:runtime.js]", FastString::from_static(include_str!("./runtime.js")))?;

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;

    result.await
}

#[op2(async)]
#[string]
pub async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op2(async)]
pub async fn op_write_file(#[string] path: String, #[string] contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op2(fast)]
pub fn op_remove_file(#[string] path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

#[op2(async)]
#[string]
pub async fn op_fetch(#[string] url: String) -> Result<String, AnyError> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[op2]
#[string]
pub fn op_test_data_in_out(#[string] mut message: String) -> Result<String, AnyError> {
    message.push_str("... and here's more");
    Ok(message)
}
