use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Implements the `FromStr` trait for enums using a greedy parsing strategy.
///
/// This custom derive macro attempts to parse a string into any of the specified enum variants
/// by trying each variant's parsing logic in declaration order. It stops at the first successful parse.
///
/// ## Usage
///
/// Derive `FromStr` on an enum where each variant either wraps a type that implements `FromStr`
/// or is a basic type that can be directly parsed from a string.
///
/// ## Example
/// ```rust
/// use greedy_enum::FromStr;
///
/// #[derive(FromStr, Debug, PartialEq)]
/// enum MyEnum {
///     Int(i32),
///     Float(f32),
///     Text(String),
/// }
///
/// let my_int = "42".parse::<MyEnum>().unwrap();
/// assert_eq!(my_int, MyEnum::Int(42));
///
/// let my_float = "3.14".parse::<MyEnum>().unwrap();
/// assert_eq!(my_float, MyEnum::Float(3.14));
///
/// let my_text = "hello".parse::<MyEnum>().unwrap();
/// assert_eq!(my_text, MyEnum::Text(String::from("hello")));
/// ```
///
/// This macro simplifies parsing logic in applications where enums represent different types or formats.
#[proc_macro_derive(FromStr)]
pub fn from_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match input.data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("FromStr can only be implemented for enum"),
    };

    let try_from_strs = variants.iter().map(|variant| {
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field_type = &fields.unnamed.first().unwrap().ty;
                quote! {
                    if let Ok(parsed) = <#field_type as std::str::FromStr>::from_str(s) {
                        return Ok(#name::#ident(parsed));
                    }
                }
            },
            _ => panic!("FromStr can only be implemented for enums with exactly one unnamed field per variant"),
        }
    });

    let expanded = quote! {
        impl ::std::str::FromStr for #name {
            type Err = ::greedy_enum::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #(#try_from_strs)*
                Err(::greedy_enum::ParseError { span: s.to_string() })
            }
        }
    };

    TokenStream::from(expanded)
}
