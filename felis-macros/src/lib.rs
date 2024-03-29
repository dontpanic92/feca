#![feature(let_chains)]

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput, FieldsNamed, Lit};

#[proc_macro_derive(FelisStyle, attributes(prop))]
pub fn style(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let (prop_name, field_name) = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                named
                    .iter()
                    .filter_map(|f| {
                        f.attrs
                            .iter()
                            .find(|a| {
                                a.path
                                    .get_ident()
                                    .map(|ident| ident == "prop")
                                    .unwrap_or(false)
                            })
                            .map(|attr| {
                                let tokens: Vec<proc_macro2::TokenTree> = attr.tokens.clone().into_iter().collect();
                                if tokens.len() == 2 && let proc_macro2::TokenTree::Literal(literal) = &tokens[1] {
                                    let lit = Lit::new(literal.clone());
                                    if let Lit::Str(litstr) = lit {
                                        return (litstr.value(), f.ident.clone().unwrap());
                                    } else {
                                        panic!("Expect a string to be the value of 'prop'")
                                    }
                                } else {
                                    let ident_name = f.ident.clone().unwrap().to_string();
                                    return (ident_name.replace('_', "-"), f.ident.clone().unwrap());
                                }
                            })
                    })
                    .unzip()
            }
            _ => (vec![], vec![]),
        },
        _ => (vec![], vec![]),
    };

    let output = quote! {
    impl #ident {
        pub fn from_key_value_list<P: AsRef<str>>(list: &[(P, P)]) -> Self {
            let mut style = Self {
                ..Default::default()
            };

            for (key, value) in list {
                let key: &str = (*key).as_ref();
                let value: &str = (*value).as_ref();
                match key {
                    #( #prop_name => style. #field_name = Some(value.parse().unwrap_or_default()) ,)*
                    _ => {}
                }
            }

            style
        }

        pub fn from_property_list(list: &[crate::style::parser::Property]) -> Self {
            let mut style = Self {
                ..Default::default()
            };

            for property in list {
                match property.key.as_str() {
                    #( #prop_name => style. #field_name = Some(property.into()) ,)*
                    _ => {}
                }
            }

            style
        }

        pub fn merge(child: &Style, parent: &Style) -> Self {
            let mut ret = Style::default();

            #( ret. #field_name= child. #field_name .clone().or_else(|| parent. #field_name .clone()); )*

            ret
        }
    }
    };

    output.into()
}

#[proc_macro_derive(FelisDefault, attributes(default_item))]
pub fn felis_default(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let field_name = match data {
        syn::Data::Enum(DataEnum { variants, .. }) => variants
            .iter()
            .filter_map(|f| {
                f.attrs
                    .iter()
                    .find(|a| {
                        a.path
                            .get_ident()
                            .map(|ident| ident == "default_item")
                            .unwrap_or(false)
                    })
                    .map(|_| f.clone())
            })
            .next()
            .expect("No item marked as default"),
        _ => panic!("Cannot be used in types other than enums"),
    }
    .ident;

    let output = quote! {
    impl ::core::default::Default for #ident {
        fn default() -> Self {
            Self::#field_name
        }
    }
    };

    output.into()
}
