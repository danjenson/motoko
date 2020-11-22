use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, AttributeArgs, Error, Ident, ItemImpl,
    Meta, NestedMeta, Type,
};

#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
    let pks = parse_macro_input!(attr as AttributeArgs)
        .iter()
        .map(|a| match a {
            NestedMeta::Meta(Meta::Path(path)) => {
                path.segments.first().unwrap().ident.clone()
            }
            _ => panic!("Invalid primary key argument"),
        })
        .collect::<Vec<Ident>>();
    let mut item_impl = parse_macro_input!(item as ItemImpl);
    let (_self_ty, self_name) = match item_impl.self_ty.as_ref() {
        Type::Path(path) => (
            path,
            path.path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap(),
        ),
        _ => panic!(Error::new_spanned(&item_impl.self_ty, "Invalid type")),
    };
    item_impl.items.insert(
        0,
        parse_quote! {
            pub async fn id(&self) -> GQLResult<ID> {
                let pk_str = vec![#(self.#pks.to_string()),*].join(":");
                Ok(base64::encode(format!("{}:{}", #self_name, pk_str)).into())
            }
        },
    );
    quote!(#item_impl).into()
}
