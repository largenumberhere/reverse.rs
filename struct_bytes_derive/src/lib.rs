use proc_macro::TokenStream;
use syn::{DeriveInput, Ident};

// The derive macro for StructToBytes trait
#[proc_macro_derive(StructToBytes)]
pub fn to_bytes_derive(items: TokenStream) ->TokenStream{
    // Convert the token stream to an abstract syntax tree which is easier to work with
    let ast: DeriveInput =syn::parse(items).unwrap();

    // Do the thing
    impl_to_bytes_trait(ast)
}

fn impl_to_bytes_trait(ast: DeriveInput)-> TokenStream {
    // Get struct name
    let identifier = ast.ident;

    // Get struct fields
    let field_identifiers: Vec<Ident> =match ast.data {
        syn::Data::Struct(s) => s.fields.into_iter().filter_map(|f| f.ident).collect(),
        syn::Data::Enum(_)=> panic!("Enums are not supported"),
        syn::Data::Union(_) => panic!("Unions are not supported")
    };

    // Generate the trait implementation. Each field_identifier has its bytes extracted and added to the vec, then the vec is returned
    let stream:TokenStream =quote::quote!{
        use crate::into_bytes::IntoBytes;

        impl ToBytes for #identifier {
            fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
                {
                    let mut bytes = vec![];
                    #( self.#field_identifiers.into_bytes(&mut bytes)?; )*;

                    Ok(bytes)
                }
            }
        }
    }
    //Convert back into a TokenStream
    .into();

    stream
}
// The derive macro for StructFromBytes trait
#[proc_macro_derive(StructFromBytes)]
pub fn from_bytes_derive(item: TokenStream) -> TokenStream{
    // Convert the token stream to an abstract syntax tree which is easier to work with
    let ast: DeriveInput = syn::parse(item).unwrap();

    // Do the thing
    impl_struct_from_bytes_trait(ast)
}

fn impl_struct_from_bytes_trait(ast: DeriveInput) -> TokenStream{
    //Get the name of the struct
    let identifier = ast.ident;

    // Get struct fields
    let field_identifiers: Vec<Ident> =match ast.data {
        syn::Data::Struct(s) => s.fields.into_iter().filter_map(|f| f.ident).collect(),
        syn::Data::Enum(_)=> panic!("Enums are not supported"),
        syn::Data::Union(_) => panic!("Unions are not supported")
    };

    // Generate the implementation. Each struct field is converted with something like `field = reader.read_primitive()`
    let stream:TokenStream =quote::quote!{
        use crate::read_primitives::ReadPrimitive;
        use std::io::Read;

        impl<R: Read> StructFromBytes<#identifier, R, std::io::Error> for #identifier {
            fn struct_from_bytes(reader: &mut R) -> Result<#identifier, std::io::Error> {
                // Make each field of the struct calculated from read_primitives::from_bytes(&mut reader)
                {
                    let struc = #identifier {
                        #(
                            #field_identifiers : reader.read_primitive()?,
                        )*
                    };

                    Ok(struc)
                }
            }
        }
    }.into();

    stream
}

