extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(MyDefault)]
pub fn my_default(input: TokenStream) -> TokenStream {
    // 解析输入的 Rust 代码
    let ast = parse_macro_input!(input as DeriveInput);

    // 提取结构体的名称
    let name = &ast.ident;

    // 生成实现 Default trait 的 Rust 代码
    let gen = match &ast.data {
        Data::Struct(data) => match &data.fields {
            // 标准结构体
            Fields::Named(fields) => {
                let field_defaults = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    quote! { #field_name: Default::default() }
                });

                quote! {
                    impl Default for #name {
                        fn default() -> Self {
                            Self {
                                #( #field_defaults, )*
                            }
                        }
                    }
                }
            }
            // 元组结构体
            Fields::Unnamed(fields) => {
                let field_defaults = (0..fields.unnamed.len()).map(|_| {
                    quote! { Default::default() }
                });

                quote! {
                    impl Default for #name {
                        fn default() -> Self {
                            Self(
                                #( #field_defaults, )*
                            )
                        }
                    }
                }
            }
            // 单元结构体
            Fields::Unit => {
                quote! {
                    impl Default for #name {
                        fn default() -> Self {
                            Self
                        }
                    }
                }
            }
        },
        _ => panic!("Only structs are supported."),
    };

    // 返回生成的 Rust 代码作为 TokenStream
    gen.into()
}
