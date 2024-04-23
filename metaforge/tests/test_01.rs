use std::io::{Read, Write};
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};

#[test]
fn deno_process() {

    #[derive(Debug, Serialize, Deserialize)]
    struct Foo {
        a: i32,
        b: i32,
        c: i32,
    }

    let (a,b,c) = (11,22,33);
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