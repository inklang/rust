use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Lit, Token};

struct GetPropertyArgs {
    obj: Expr,
    key: Expr,
}

impl Parse for GetPropertyArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let obj = input.parse()?;
        _ = input.parse::<Token![,]>()?;
        let key = input.parse()?;

        Ok(GetPropertyArgs { obj, key })
    }
}

#[proc_macro]
pub fn get_property(input: TokenStream) -> TokenStream {
    let GetPropertyArgs { obj, key } = parse_macro_input!(input as GetPropertyArgs);

    let expanded = quote! {
        #obj[#key].take()
    };

    TokenStream::from(expanded)
}
