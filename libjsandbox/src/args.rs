use deno_core::{error, serde, serde_json};

/// 参数特征
pub trait Args {
    fn to_string(self) -> Result<String, error::AnyError>;
}

/// 空参数
impl Args for () {
    fn to_string(self) -> Result<String, error::AnyError> {
        Ok(String::new())
    }
}

/// 参数实现宏
macro_rules! args_impl {
  ($($var:ident),+) => {
    #[allow(non_snake_case)]
    impl<$($var: serde::Serialize),+> Args for ($($var),+,) {
      fn to_string(self) -> Result<String, error::AnyError> {
        let ($($var),+,) = self;
        Ok([
          $(serde_json::to_value($var)?.to_string()),+
        ].join(","))
      }
    }
  };
}

// 应用参数实现宏
args_impl!(T0);
args_impl!(T0, T1);
args_impl!(T0, T1, T2);
args_impl!(T0, T1, T2, T3);
args_impl!(T0, T1, T2, T3, T4);
args_impl!(T0, T1, T2, T3, T4, T5);
args_impl!(T0, T1, T2, T3, T4, T5, T6);
args_impl!(T0, T1, T2, T3, T4, T5, T6, T7);
args_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
args_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
