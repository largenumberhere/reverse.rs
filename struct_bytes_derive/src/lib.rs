use proc_macro::TokenStream;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(StructToBytes)] //
pub fn to_bytes_derive(items: TokenStream) ->TokenStream{
    let ast: DeriveInput =syn::parse(items).unwrap();
    impl_to_bytes_trait(ast)

}

fn impl_to_bytes_trait(ast: DeriveInput)-> TokenStream {
    // Get struct name
    let identifier = ast.ident;
    //let identifier_string = identifier.to_string();

    // Get struct fields
    let field_identifiers: Vec<Ident> =match ast.data {
        syn::Data::Struct(s) => s.fields.into_iter().filter_map(|f| f.ident).collect(),
        syn::Data::Enum(_)=> panic!("Enums are not supported"),
        syn::Data::Union(_) => panic!("Unions are not supported")
    };

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
    }.into();

    stream
}

#[proc_macro_derive(StructFromBytes)]
pub fn from_bytes_derive(item: TokenStream) -> TokenStream{
    let ast: DeriveInput = syn::parse(item).unwrap();
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

