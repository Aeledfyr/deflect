use std::{fmt, ops};

/// A field of a [struct][super::Struct] or [variant][super::Variant].
pub struct Field<'value, 'dwarf, R: crate::gimli::Reader<Offset = usize>>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    schema: crate::schema::Field<'dwarf, R>,
    value: crate::Bytes<'value>,
}

impl<'value, 'dwarf, R> Field<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn new(
        schema: crate::schema::Field<'dwarf, R>,
        value: crate::Bytes<'value>,
    ) -> Self {
        Self { schema, value }
    }

    /// The value of this field.
    pub fn value(&'dwarf self) -> Result<super::Value<'value, 'dwarf, R>, crate::Error> {
        let r#type = self .r#type()?;
        let offset = self
            .offset()?
            .ok_or(crate::ErrorKind::MissingAttr {
                attr: crate::gimli::DW_AT_type,
            })?
            .address(0)? as usize;
        let value = &self.value[offset..];
        Ok(unsafe { super::Value::with_type(r#type, value) })
    }
}

impl<'value, 'dwarf, R> fmt::Display for Field<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Ok(Some(field_name)) => field_name.fmt(f)?,
            Ok(None) => panic!("field does not have a name"),
            Err(err) => panic!("reader error: {:?}", err),
        };
        f.write_str(" : ")?;
        match self.value() {
            Ok(value) => value.fmt(f),
            Err(err) => panic!("reader error: {:?}", err),
        }
    }
}

impl<'value, 'dwarf, R> ops::Deref for Field<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    type Target = crate::schema::Field<'dwarf, R>;

    fn deref(&self) -> &Self::Target {
        &self.schema
    }
}
