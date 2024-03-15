// main.rs
use std::rc::Rc;
use deno_ast::{MediaType, ParseParams, SourceTextInfo};

use deno_core::{Extension, FastString, ModuleLoadResponse, ModuleSourceCode, ModuleSpecifier, Op, op2, PollEventLoopOptions, RequestedModuleType};
use deno_core::error::AnyError;

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js("metaforge/examples/rust_to_js/example.ts")) {
    // if let Err(error) = runtime.block_on(run_js("metaforge/examples/rust_to_js/my_module.js")) {
        eprintln!("error: {}", error);
    }
}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, &*std::env::current_dir().unwrap())?;

    let runjs_extension = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[op_read_file::DECL, op_write_file::DECL, op_remove_file::DECL, op_fetch::DECL]),
        ..Default::default()
    };


    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(TsModuleLoader)),
        // module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    js_runtime.execute_script("[runjs:runtime.js]", FastString::from_static(include_str!("./runtime.js")))?;

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(PollEventLoopOptions::default()).await?;
    let rst = result.await?;
    Ok(rst)
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


struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, deno_core::error::AnyError> {
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
    }

    fn load(&self, module_specifier: &ModuleSpecifier, _maybe_referrer: Option<&ModuleSpecifier>, _is_dyn_import: bool, _requested_module_type: RequestedModuleType) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        ModuleLoadResponse::Sync({
            let path = module_specifier.to_file_path().unwrap();

            // 根据文件扩展名解析 MediaType，据此判断是否需要对文件进行编译
            let media_type = MediaType::from_path(&path);
            let (module_type, should_transpile) = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    (deno_core::ModuleType::JavaScript, false)
                }
                MediaType::Jsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::TypeScript
                | MediaType::Mts
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => (deno_core::ModuleType::JavaScript, true),
                MediaType::Json => (deno_core::ModuleType::Json, false),
                _ => panic!("Unknown extension {:?}", path.extension()),
            };

            // 读取文件，在需要的情况下对文件进行转译
            let string_code = std::fs::read_to_string(&path).unwrap();
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.to_string(),
                    text_info: SourceTextInfo::from_string(string_code),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                }).unwrap();
                let string = parsed.transpile(&Default::default()).unwrap().text;
                string
            } else {
                string_code
            };
            let code = ModuleSourceCode::String(FastString::from(code));

            // 加载模块并返回
            let module = deno_core::ModuleSource::new(module_type, code, &module_specifier, );
            Ok(module)
        })
    }
}