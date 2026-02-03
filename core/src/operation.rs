use openapiv3::{Operation, Parameter, RefOr, Schema, SchemaKind, Type};

pub struct OperationRegister {
    pub name: &'static str,
    pub constructor: &'static (dyn Sync + Send + Fn() -> Operation),
}

pub trait OaParameter {
    fn body_schema() -> Option<RefOr<Schema>> {
        None
    }
    fn parameter_schemas() -> Vec<RefOr<Schema>> {
        Vec::new()
    }
    fn parameters() -> Vec<RefOr<Parameter>> {
        Vec::new()
    }
}

impl<T, E> OaParameter for Result<T, E>
where
    T: OaParameter,
{
    fn body_schema() -> Option<RefOr<Schema>> {
        T::body_schema()
    }
}

inventory::collect!(OperationRegister);

pub fn query_parameters<T: OaParameter>() -> Vec<RefOr<openapiv3::Parameter>> {
    T::parameter_schemas()
        .into_iter()
        .flat_map(|s| s.into_item())
        .flat_map(|s| match s.kind {
            SchemaKind::Type(Type::Object(o)) => Some(o.properties),
            _ => None,
        })
        .flatten()
        .map(|(k, v)| RefOr::Item(openapiv3::Parameter::query(k, v)))
        .collect()
}

#[cfg(all(feature = "qs", any(feature = "axum", feature = "actix")))]
impl<T: OaParameter> OaParameter for serde_qs::web::QsQuery<T> {
    fn parameters() -> Vec<RefOr<openapiv3::Parameter>> {
        query_parameters::<T>()
    }
}
