use std::env;
use std::ops::Mul;
// main.rs
use std::rc::Rc;
use std::time::Instant;

use deno_core::{Extension, FastString, JsRuntime, Op, op2, PollEventLoopOptions, v8};
use deno_core::anyhow::Error;
use deno_core::error::{AnyError, generic_error};
use deno_core::v8::Handle;
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use libdatabase::sqlx::ColumnIndex;
use libtracing::error;

// #[tokio::main]
fn main() -> Result<(), deno_core::anyhow::Error> {

    // let contents = tokio::fs::read_to_string("/mnt/gr01/RustroverProjects/myaxumserver/metaforge/examples/rust_to_js/log.txt").await?;
    // dbg!(&contents);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(call_js_file_05()) {
        eprintln!("error: {error}");
    }

    // let string = call_js_file_05().await?;
    // println!("result: {:?}", string);




    Ok(())
}

// 执行JS文件中的异步函数
pub async fn call_js_file_05() -> Result<String, deno_core::anyhow::Error> {
    // 初始化扩展方法
    let runjs_extension = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[
            op_test_data_in_out::DECL,
            op_read_file::DECL,
            op_write_file::DECL,
            op_remove_file::DECL,
            op_fetch::DECL,
            integer_x_3::DECL,
            float_x_3::DECL,
            true_to_false::DECL,
            vec_to_vec::DECL,
            struct_to_struct::DECL,
            struct_to_struct_01::DECL,
        ]),
        ..Default::default()
    };

    // 为 js 运行时添加扩展接口
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    // 初始化全局变量
    js_runtime.execute_script("[runjs:runtime.js]", FastString::from_static(include_str!("./runtime.js")))?;

    let res = js_runtime.execute_script(
        "call-back-to-rust",
        FastString::from_static(include_str!("./output_05.js")))?;

    let res = js_runtime.execute_script(
        "call-back-to-rust",
        r#"
        (async ()=>{
            // let y1 = await output_05.gr_struct();
            if(!!(output_05.jsReadFile)){
                console.log("has a jsReadFile!!!")
            }
            let y1 = output_05.jsReadFile("/mnt/gr01/RustroverProjects/myaxumserver/metaforge/examples/rust_to_js/log.txt");
            return "qqqqqqqqqqqqq";
        })();
        "#)?;

    let value = res.open(js_runtime.v8_isolate());
    if value.is_promise() {
        print!("yes it is a promise\n");
    }

    let resolve = js_runtime.resolve(res);
    let promise_result = js_runtime.with_event_loop_promise(resolve, PollEventLoopOptions::default()).await;
    let str = promise_result?.open(js_runtime.v8_isolate()).to_rust_string_lossy(&mut js_runtime.handle_scope());
    dbg!(&str);
    // let str1: mini = serde_json::from_str(&str)?;
    // dbg!(&str1);

    Ok(str)
}


// 执行JS文件中的同步函数
pub async fn call_js_file_04() -> Result<String, deno_core::anyhow::Error> {
    // 初始化扩展方法
    let runjs_extension = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[
            op_test_data_in_out::DECL,
            op_read_file::DECL,
            op_write_file::DECL,
            op_remove_file::DECL,
            op_fetch::DECL,
            integer_x_3::DECL,
            float_x_3::DECL,
            true_to_false::DECL,
            vec_to_vec::DECL,
            struct_to_struct::DECL,
            struct_to_struct_01::DECL,
        ]),
        ..Default::default()
    };

    // 为 js 运行时添加扩展接口
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    // 初始化全局变量
    js_runtime.execute_script("[runjs:runtime.js]", FastString::from_static(include_str!("./runtime.js")))?;

    let res = js_runtime.execute_script(
        "call-back-to-rust",
        FastString::from_static(include_str!("./output_06.js")))?;

    let res = js_runtime.execute_script(
        "call-back-to-rust",
        r#"
        (()=>{
            let y1 = output_06.gr_struct_01();
            return y1;
        })();
        "#)?;

    let value = res.open(js_runtime.v8_isolate());
    if value.is_promise() {
        print!("yes it is a promise\n");
    }

    let resolve = js_runtime.resolve(res);
    let promise_result = js_runtime.with_event_loop_promise(resolve, PollEventLoopOptions::default()).await;
    let str = promise_result?.open(js_runtime.v8_isolate()).to_rust_string_lossy(&mut js_runtime.handle_scope());
    dbg!(&str);
    let str1: mini = serde_json::from_str(&str)?;
    dbg!(&str1);

    Ok(str)
}


// 运行一个自动执行的 JS 脚本
pub async fn call_js_file_03() -> Result<String, deno_core::anyhow::Error> {
    // 创建默认配置JS运行时
    let mut runtime = JsRuntime::new(Default::default());
    // 从 JS 脚本加载函数
    let res = runtime.execute_script(
        "output_01",
        FastString::from_static(include_str!("./output_01.js")))?;
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
    // let str = res.open(runtime.v8_isolate()).to_rust_string_lossy(&mut runtime.handle_scope());
    let resolve = runtime.resolve(res);
    let promise_result = runtime.with_event_loop_promise(resolve, PollEventLoopOptions::default()).await;
    let str = promise_result?.open(runtime.v8_isolate()).to_rust_string_lossy(&mut runtime.handle_scope());

    // 返回结果
    Ok(str)
}


// 运行一个自动执行的 JS 脚本
pub async fn call_js_file_02() -> Result<String, deno_core::anyhow::Error> {
    // 创建默认配置JS运行时
    let mut runtime = JsRuntime::new(Default::default());
    // 从 JS 脚本加载函数
    let res = runtime.execute_script(
        "output_04",
        FastString::from_static(include_str!("./output_04.js")))?;
    // 执行脚本里的函数，获取执行结果
    let res = runtime.execute_script(
        "output_04",
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
        FastString::from_static(include_str!("./output_03.js")))?;
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


pub async fn do_has_param_func_01() -> Result<String, deno_core::anyhow::Error> {
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

    let res = js_runtime.execute_script(
        "how-long-till-lunch.esm1",
        FastString::from_static(include_str!("./my_module.mjs")))?;

    let start_time = Instant::now();
    let str = res.open(js_runtime.v8_isolate())
        .to_rust_string_lossy(&mut js_runtime.handle_scope());

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
    Ok(str)
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


#[derive(Deserialize, Serialize, Debug)]
pub struct mini {
    id: i64,
    pub name: String,
}

#[op2(async)]
#[serde]
pub async fn struct_to_struct(#[serde] mut input: mini) -> Result<mini, AnyError> {
    input.name = "郭睿".into();
    Ok(input)
}

#[op2]
#[serde]
pub fn struct_to_struct_01(#[serde] mut input: mini) -> Result<mini, AnyError> {
    input.name = "郭睿_01".into();
    Ok(input)
}

#[op2(async)]
#[serde]
pub async fn vec_to_vec(#[serde] mut input: Vec<i32>) -> Result<Vec<i32>, AnyError> {
    input.push(1000);
    Ok(input)
}

#[op2(async)]
pub async fn true_to_false(input: bool) -> Result<bool, AnyError> {
    Ok(!input)
}

#[op2(async)]
pub async fn float_x_3(input: f64) -> Result<f64, AnyError> {
    Ok(input.mul(3f64))
}

// #[op2(fast)]
#[op2(async)]
pub async fn integer_x_3(input: i32) -> Result<i32, AnyError> {
    input.checked_mul(3).ok_or(generic_error("error in integer_x_3."))
}

#[op2(async)]
#[string]
pub async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    dbg!(&path);
    let contents = tokio::fs::read_to_string(path).await;
    dbg!(&contents);
    Ok(contents.unwrap())
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
    let body = "reqwest::get(url).await?.text().await?".to_string();
    Ok(body)
}

#[op2]
#[string]
pub fn op_test_data_in_out(#[string] mut message: String) -> Result<String, AnyError> {
    message.push_str("... and here's more");
    Ok(message)
}
