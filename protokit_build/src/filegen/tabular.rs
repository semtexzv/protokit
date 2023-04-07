use core::str::FromStr;
use anyhow::bail;
use quote::__private::TokenStream;
use quote::quote;
use protokit_desc::{DataType, FieldDef, Frequency};
use protokit_proto::translate::TranslateCtx;
use crate::filegen::Options;

pub fn tabular_parser(ctx: &TranslateCtx, opts: &Options, f: &FieldDef, i: usize) -> Result<TokenStream> {
    use crate::BuiltinType::*;
    let str = TokenStream::from_str(&type_to_str(ctx, opts, &f.typ)?).unwrap();
    Ok(match &f.typ {
        DataType::Builtin(String_ | Bytes_) if f.is_repeated() => quote!( Bytes::<Vec<#str>, #i> ),
        DataType::Builtin(String_ | Bytes_) => quote!( Bytes::<#str, #i> ),

        DataType::Builtin(bt) if bt.is_varint() && bt.is_zigzag() && f.is_repeated_packed()  => {
            quote! { PackedSInt::<#str, #i> }
        }
        DataType::Builtin(bt) if bt.is_varint() && bt.is_zigzag() => {
            quote! { SInt::<#str, #i> }
        }
        DataType::Builtin(bt) if bt.is_varint() && f.is_repeated_packed() => {
            quote! { PackedVInt::<#str, #i> }
        }
        DataType::Builtin(bt) if bt.is_varint() => {
            quote! { VInt::<#str, #i> }
        }
        DataType::Builtin(_bt) if f.is_repeated_packed() => {
            quote! { PackedFixed::<#str, #i> }
        }
        DataType::Builtin(_bt) => {
            quote! { Fixed::<#str, #i> }
        }
        DataType::Message(_m) if f.is_repeated() => {
            quote! { Repeated::<#str, #i> }
        }
        DataType::Message(_m) => {
            quote! { Nested::<#str, #i> }
        }
        DataType::Enum(_m) if f.is_repeated() => {
            quote! { Enum::<#str, #i> }
        }
        DataType::Enum(_m) => {
            quote! { Enum::<#str, #i> }
        }
        DataType::Map(m) => {
            let kf = FieldDef {
                name: Default::default(),
                frequency: Frequency::Normal,
                typ: DataType::Builtin(m.0.clone()),
                num: 0,
                #[cfg(feature = "descriptors")]
                options: Default::default(),
            };
            let kp = tabular_parser(ctx, opts, &kf, 1)?;
            let _vf = FieldDef {
                name: Default::default(),
                frequency: Frequency::Normal,
                typ: m.1.clone(),
                num: 0,
                #[cfg(feature = "descriptors")]
                options: Default::default(),
            };
            let vp = tabular_parser(ctx, opts, &kf, 1)?;
            quote! { Mapped::<binformat::tabular::#kp, binformat::tabular::#vp, #i> }
        }
        other => bail!("Unknown: {other:?}"),
    })
}

// let tabular_fields = tabular_fields.values();
// let tabular_format = if cfg!(feature = "tabular") {
//     quote! {
//         impl binformat::tabular::TableDecodable for #msg_name {
//             fn info(&self) -> binformat::tabular::MessageInfo {
//                 const _INFO: &[binformat::tabular::FieldInfo] = &[
//                     #(#tabular_fields,)*
//                 ];
//                 binformat::tabular::MessageInfo::sorted(_INFO, #qualified_name)
//             }
//         }
//     }
// } else {
//     quote! {}
// };

pub fn gnerate_tabular_field() {
    let taglen = u64::required_space(normal_tag as u64);
    // TODO: Use tag len here, that's the intended usecase
    let tabular = tabular_parser(ctx, opts, &field, taglen).unwrap();
    let encoded_tag = protokit_binformat::tabular::tag(normal_tag);
    let field_num = field_idx as u32;
    out.tabular_fields.insert(encoded_tag,quote! {
        binformat::tabular::FieldInfo {
            tag: #encoded_tag,
            offset: binformat::tabular::offset_of!(#msg_name, #name) as isize,
            parser: <binformat::tabular::#tabular as binformat::tabular::Format>::decode,
            number: #field_num,
        }
    });

}