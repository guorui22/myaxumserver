use serde::{Deserialize, Serialize};
use tonic::codegen::Body;

use metaforge::util;

#[tokio::test]
async fn deno_process_with_args() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Foo {
        a: i32,
        b: i32,
        c: i32,
    }

    let (a, b, c) = (110, 220, 330);
    let f1 = Foo { a, b, c };

    let args = serde_json::json!(f1);
    let script = r#"
        import _ from 'npm:lodash@4.17.21';

        function process(obj) {
            let arr = [];
            _.forIn(obj, function(value, key) {
                arr.push(value);
            });
            return arr;
        }
    "#;

    let result = util::deno::js_deno_cli(String::from(script), "process".to_string(), args).unwrap();
    let arr:Vec<i32> = serde_json::from_str(&result).unwrap();
    dbg!(&arr);
}
