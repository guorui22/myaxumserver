use std::io::{Read, Write};
use std::process::{Command, Stdio};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tera::Tera;
use tokio::io::AsyncReadExt;

#[tokio::test]
async fn deno_process() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Foo {
        a: i32,
        b: i32,
        c: i32,
    }

    let (a, b, c) = (11, 22, 33);
    let f1 = Foo { a, b, c };

    // 启动 Deno 进程
    let mut deno_process = Command::new("deno")
        .arg("run")
        .arg("--unstable-net")
        .arg("--allow-net")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start deno process");

    let f2 = serde_json::json!(f1).to_string();

    // 向 Deno 发送命令
    let input = format!(r#"
import _ from 'npm:lodash@4.17.21';

// 遍历对象的所有属性，包括原型链上的属性，并返回所有属性值组成的数组
function for_in_object(obj) {{
    let arr = [];
    _.forIn(obj, function(value, key) {{
        arr.push(value);
    }});
    return arr;
}}

let arr = for_in_object({f2});

console.log(JSON.stringify(arr));

    "#);
    if let Some(mut stdin) = deno_process.stdin.take() {
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
    }

    // 读取 Deno 的输出
    let mut output = String::new();
    deno_process.stdout.as_mut().expect("Failed to get stdout").read_to_string(&mut output).expect("Failed to read from stdout");

    // 等待 Deno 进程结束
    let deno_result = deno_process.wait().expect("Failed to wait for deno process");

    let mut error_output = String::new();
    deno_process.stderr.as_mut().expect("Failed to get stderr").read_to_string(&mut error_output).expect("Failed to read from stderr");

    // 打印 Deno 的输出和退出状态
    println!("Deno output: {}", output);
    println!("Deno error: {}", error_output);
    println!("Deno exited with: {}", deno_result);
}

#[tokio::test]
async fn deno_process_with_args() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Foo {
        a: i32,
        b: i32,
        c: i32,
    }

    let (a, b, c) = (11, 22, 33);
    let f1 = Foo { a, b, c };

    let args = serde_json::json!(f1);
    let script = r#"
import _ from 'npm:lodash@4.17.21';

function process(obj: any) {
    let arr = [];
    _.forIn(obj, function(value, key) {
        arr.push(value);
    });
    return arr;
}
    "#;

    let result = js_deno_cli(String::from(script), args).unwrap();
    dbg!(&result.to_string());
}

pub fn js_deno_cli(script: String, args: serde_json::Value) -> anyhow::Result<serde_json::Value> {
    let mut deno_process = Command::new("deno")
        .arg("run")
        .arg("--allow-all")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut tera = Tera::default();
    let script = format!(r#"
        {script}

        let result = process({{{{args}}}});
        console.log(JSON.stringify(result));
    "#);
    dbg!(&script);
    tera.add_raw_template("script", script.as_str())?;
    let mut context = tera::Context::new();
    context.insert("args", args.to_string().as_str());
    let rendered_script = tera.render("script", &context)?;

    dbg!(&rendered_script);

    if let Some(mut stdin) = deno_process.stdin.take() {
        stdin.write_all(rendered_script.as_bytes())?;
    }
    let is_ok = !deno_process.wait()?.success();
    if is_ok {
        let mut string_buffer = String::new();
        if deno_process.stderr.as_mut().ok_or(anyhow!("Failed to get stderr."))?.read_to_string(&mut string_buffer)? > 0 {
            return Err(anyhow!(string_buffer));
        }
    }
    if let Some(stdout) = deno_process.stdout.as_mut() {
        let mut string_buffer = String::new();
        if stdout.read_to_string(&mut string_buffer)? > 0 {
            return Ok(serde_json::json!(string_buffer));
        }
    }

    Ok(serde_json::Value::Null)
}