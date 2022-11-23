use std::fmt;

pub struct Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    value: crate::Bytes<'value>,
    schema: crate::schema::array<'dwarf, R>,
}

impl<'value, 'dwarf, R> Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) unsafe fn with_schema(
        value: crate::Bytes<'value>,
        schema: crate::schema::array<'dwarf, R>,
    ) -> Result<Self, crate::error::Error> {
        let len = schema.n()?;
        let elt_size = schema.t()?.size()?;
        let bytes = len.checked_mul(elt_size).unwrap();
        let bytes = usize::try_from(bytes).unwrap();
        let value = &value[..bytes];
        Ok(Self { value, schema })
    }

    pub fn iter(
        &self,
    ) -> Result<
        impl Iterator<Item = Result<super::Value<'value, 'dwarf, R>, crate::error::Error>>,
        crate::error::Error,
    > {
        let elt_type = self.schema.t()?;
        let elt_size = elt_type.size()?;
        let elt_size = usize::try_from(elt_size).unwrap();
        Ok(self
            .value
            .chunks(elt_size)
            .map(move |chunk| unsafe { super::Value::with_type(elt_type.clone(), chunk) }))
    }
}

impl<'value, 'dwarf, R> fmt::Debug for Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::Array");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, R> fmt::Display for Array<'value, 'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();
        for maybe_elt in self.iter().unwrap() {
            let elt = maybe_elt.unwrap();
            debug_list.entry(&crate::DebugDisplay(elt));
        }
        debug_list.finish()
    }
}
