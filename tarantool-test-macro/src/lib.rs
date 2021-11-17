use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn tarantool_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let fn_input = input.clone();
    let function = parse_macro_input!(fn_input as ItemFn);

    let name = function.sig.ident;
    let collect_func = match function.sig.output {
        syn::ReturnType::Default => "__collect_test_void",
        syn::ReturnType::Type(..) => "__collect_test_with_result",
    };
    let mut code = format!("");
    code += &format!("#[ctor::ctor]\n");
    code += &format!("fn _{name}__init() {{ tarantool_test::{}(module_path!(), \"{name}\", {name}) }}\n", collect_func, name=name);
    code += &input.to_string();
    code.parse().unwrap()
}

#[proc_macro_attribute]
pub fn tnt_test(args: TokenStream, input: TokenStream) -> TokenStream {
    tarantool_test(args, input)
}
