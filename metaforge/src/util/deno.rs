use std::io::{Read, Write};
use std::process::{Command, Stdio};
use anyhow::anyhow;
use tera::Tera;
use uuid::Uuid;

/// ### 执行 js 脚本中的指定函数
///
/// script js 脚本内容
///
/// fn_name 函数名称
///
/// arg 函数参数
pub fn js_deno_cli(script: String, fn_name: String, arg: serde_json::Value) -> anyhow::Result<String> {

    // 启动 deno 命令行子进程
    let mut deno_process = Command::new("deno")
        .arg("run")
        .arg("--allow-all") // 允许所有权限
        .arg("-") // 允许从标准输入获取脚本内容
        .stdin(Stdio::piped()) // 标准输入
        .stdout(Stdio::piped()) // 标准输出
        .stderr(Stdio::piped()) // 标准错误输出
        .spawn()?;

    // 使用 tera 模板库制作脚本模版
    let mut tera = Tera::default();
    let mut uuid_divide_line = Uuid::new_v4().to_string();
    let script = format!(r#"
        // 用户注入的 JS 数据处理脚本
        // 1.主处理函数签名必须是 function process(input)
        // 2.主处理函数的输入参数只有一个
        {script}

        // 以 JSON 字符串格式把主例函数的返回结果注入到标准输出
        // 使用 UUID 区分控制台的其他调试内容
        let result = {fn_name}({{{{args}}}});
        console.log("{uuid_divide_line}");
        console.log(JSON.stringify(result));
    "#);
    tera.add_raw_template("script", script.as_str())?;
    let mut context = tera::Context::new();
    context.insert("args", arg.to_string().as_str());
    let rendered_script = tera.render("script", &context)?;

    // 向命令行注入脚本
    if let Some(mut stdin) = deno_process.stdin.take() {
        stdin.write_all(rendered_script.as_bytes())?;
    }
    // 等待脚本执行完毕，并返回是否执行成功
    let is_success = !deno_process.wait()?.success();

    // 如果脚本执行失败，就返回控制台错误信息
    if is_success {
        let mut string_buffer = String::new();
        if deno_process.stderr.as_mut().ok_or(anyhow!("Failed to get stderr."))?.read_to_string(&mut string_buffer)? > 0 {
            return Err(anyhow!(string_buffer));
        }
    }

    // 如果脚本执行成功，就返回控制台信息
    // 通过 UUID 定位返回控制台最后的返回信息
    if let Some(stdout) = deno_process.stdout.as_mut() {
        let mut string_buffer = String::new();
        if stdout.read_to_string(&mut string_buffer)? > 0 {
            uuid_divide_line.push('\n');
            let mut v: Vec<&str> = string_buffer.split(&uuid_divide_line).collect();
            return match v.pop() {
                None => {
                    Err(anyhow!("[{}:{}]{fn_name}()函数返回信息为空!!!",file!(), line!()))
                }
                Some(s) => {
                    Ok(String::from(s))
                }
            }
        }
    }

    Err(anyhow!("运行脚本时在{}文件{}行发生未知错误!!!", file!(), line!()))
}