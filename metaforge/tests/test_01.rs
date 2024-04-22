use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[test]
fn deno_process() {
    // 启动 Deno 进程
    let mut deno_process = Command::new("deno")
        .arg("run")
        .arg("--unstable")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start deno process");

    // 向 Deno 发送命令
    let input = "console.log('Hello from Rust!');";
    if let Some(mut stdin) = deno_process.stdin.take() {
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
    }

    // 读取 Deno 的输出
    let mut output = String::new();
    deno_process.stdout.as_mut().expect("Failed to get stdout").read_to_string(&mut output).expect("Failed to read from stdout");

    // 等待 Deno 进程结束
    let deno_result = deno_process.wait().expect("Failed to wait for deno process");

    // 打印 Deno 的输出和退出状态
    println!("Deno output: {}", output);
    println!("Deno exited with: {}", deno_result);
}