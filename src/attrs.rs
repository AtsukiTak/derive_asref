use proc_macro2::Span;
use std::convert::TryFrom;

/// `#[as_ref(key = "value")]` style attribute.
pub enum Attribute {
    /// `#[as_ref(target = "value")]` style attribute
    Target(TargetAttribute),
}

impl Attribute {
    /// Returns `as_ref` in `#[as_ref(key = "value")]`.
    pub const PATH: &'static str = "as_ref";

    pub fn to_target(&self) -> Option<&TargetAttribute> {
        match self {
            Attribute::Target(ref attr) => Some(attr),
        }
    }
}

impl TryFrom<syn::Attribute> for Attribute {
    type Error = syn::Error;

    fn try_from(origin: syn::Attribute) -> syn::Result<Self> {
        AbstractAttribute::try_from(origin).and_then(Attribute::try_from)
    }
}

impl TryFrom<AbstractAttribute> for Attribute {
    type Error = syn::Error;

    fn try_from(item: AbstractAttribute) -> syn::Result<Self> {
        match item.key.to_string().as_str() {
            TargetAttribute::KEY => {
                let target_attr = TargetAttribute::try_from_lit(item.value)?;
                Ok(Attribute::Target(target_attr))
            }
            _ => Err(syn::Error::new_spanned(&item.key, "unsupported attribute")),
        }
    }
}

/// `#[as_ref(target = "value")]` style attribute
pub struct TargetAttribute {
    pub target: syn::Ident,
}

impl TargetAttribute {
    pub const KEY: &'static str = "target";

    fn try_from_lit(lit: syn::Lit) -> syn::Result<Self> {
        let target_str = match lit {
            syn::Lit::Str(s) => s.value(),
            _ => return Err(syn::Error::new_spanned(&lit, "is not string")),
        };
        Ok(TargetAttribute {
            target: syn::Ident::new(target_str.as_str(), Span::call_site()),
        })
    }
}

/// `#[as_ref(key = "value")]` style attribute.
struct AbstractAttribute {
    pub key: syn::Ident,
    pub value: syn::Lit,
}

impl TryFrom<syn::Attribute> for AbstractAttribute {
    type Error = syn::Error;

    fn try_from(origin: syn::Attribute) -> syn::Result<Self> {
        // pathが"Attribute::path"であるattributeのみ扱う
        if !origin.path.is_ident(Attribute::PATH) {
            return Err(invalid_format_err(&origin));
        }

        let mut nested_meta = match origin.parse_meta()? {
            syn::Meta::List(meta) => meta.nested,
            _ => return Err(invalid_format_err(&origin)),
        };

        if nested_meta.len() != 1 {
            return Err(invalid_format_err(&origin));
        }

        let first_inner_meta = nested_meta.pop().unwrap().into_value();

        match first_inner_meta {
            syn::NestedMeta::Meta(syn::Meta::NameValue(kv)) => match kv.path.get_ident() {
                Some(ident) => Ok(AbstractAttribute {
                    key: ident.clone(),
                    value: kv.lit,
                }),
                None => Err(invalid_format_err(&origin)),
            },
            _ => Err(invalid_format_err(&origin)),
        }
    }
}

fn invalid_format_err(origin: &syn::Attribute) -> syn::Error {
    syn::Error::new_spanned(
        origin,
        format!(
            "Invalid attribute format. We only support \"#[{}(key = \"value\")]\" style attribute",
            Attribute::PATH
        ),
    )
}
