use crate::attrs::Attribute;
use std::convert::TryFrom;

pub struct Struct {
    pub ident: syn::Ident,
    pub attrs: Vec<Attribute>,
    pub fields: Vec<Field>,
}

impl TryFrom<syn::DeriveInput> for Struct {
    type Error = syn::Error;

    fn try_from(origin: syn::DeriveInput) -> syn::Result<Struct> {
        let data = match &origin.data {
            syn::Data::Struct(ref data) => data,
            _ => return Err(syn::Error::new_spanned(&origin, "is not a struct")),
        };

        let fields = match &data.fields {
            syn::Fields::Named(ref fields) => fields
                .named
                .iter()
                .cloned()
                .map(Field::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            syn::Fields::Unnamed(ref fields) => fields
                .unnamed
                .iter()
                .cloned()
                .map(Field::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            syn::Fields::Unit => {
                return Err(syn::Error::new_spanned(origin.ident, "There is no field"))
            }
        };

        let attrs = origin
            .attrs
            .iter()
            .map(|attr| Attribute::try_from(attr.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Struct {
            ident: origin.ident,
            attrs,
            fields,
        })
    }
}

pub struct Field {
    pub ident: syn::Ident,
    pub attrs: Vec<Attribute>,
}

impl TryFrom<syn::Field> for Field {
    type Error = syn::Error;

    fn try_from(origin: syn::Field) -> syn::Result<Field> {
        let attrs = origin
            .attrs
            .iter()
            .map(|attr| Attribute::try_from(attr.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        let ident = origin.ident.clone().ok_or(syn::Error::new_spanned(
            &origin,
            "We only support named field",
        ))?;

        Ok(Field { attrs, ident })
    }
}
