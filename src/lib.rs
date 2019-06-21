extern crate proc_macro;

use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitStr, Token,
};

struct DefaultEnv {
    env_var: LitStr,
    default_value: Expr,
}

impl Parse for DefaultEnv {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let env_var: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let default_value: Expr = input.parse()?;
        Ok(Self {
            env_var,
            default_value,
        })
    }
}

#[proc_macro]
pub fn default_env(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DefaultEnv {
        env_var,
        default_value,
    } = parse_macro_input!(input as DefaultEnv);

    let var_or_default = match std::env::var(env_var.value()) {
        Ok(var) => quote! { #var },
        Err(_) => quote! { #default_value },
    };

    proc_macro::TokenStream::from(var_or_default)
}
