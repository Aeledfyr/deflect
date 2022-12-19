use std::fmt;

/// A reflected slice value.
pub struct SlicePointer<'value, 'dwarf, K, P = crate::DefaultProvider>
where
    P: crate::DebugInfoProvider,
{
    pub(crate) schema: crate::schema::Pointer<'dwarf, K, P::Reader>,
    pub(crate) value: crate::Bytes<'value>,
    pub(crate) provider: &'dwarf P,
    pub(crate) length: usize,
}

impl<'value, 'dwarf, K, P> SlicePointer<'value, 'dwarf, K, P>
where
    P: crate::DebugInfoProvider,
{
    /// The value of the `data_ptr` field of this slice.
    pub fn data_ptr(&self) -> crate::Bytes<'value> {
        self.value
    }

    /// The length of the slice
    pub fn length(&self) -> usize {
        self.length
    }

    /// An iterator over values of this slice.
    pub fn iter(&self) -> Result<super::Iter<'value, 'dwarf, P>, crate::Error> {
        let elt_type = self.schema.r#type()?;
        let elt_size = elt_type.size()?;
        let elt_size = usize::try_from(elt_size)?;

        let length = self.length();
        let bytes = elt_size * length;

        let value = self.data_ptr().as_ptr();
        let value = std::ptr::slice_from_raw_parts(value, bytes);
        let value = unsafe { &*value };

        Ok(unsafe { super::Iter::new(value, elt_size, elt_type, length, self.provider) })
    }
}

impl<'value, 'dwarf, K, P> fmt::Debug for SlicePointer<'value, 'dwarf, K, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("deflect::value::SlicePointer");
        debug_struct.field("schema", &self.schema);
        debug_struct.field("value", &self.value);
        debug_struct.finish()
    }
}

impl<'value, 'dwarf, K, P> fmt::Display for SlicePointer<'value, 'dwarf, K, P>
where
    P: crate::DebugInfoProvider,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("&")?;
        let mut debug_list = f.debug_list();
        for maybe_elt in self.iter().map_err(crate::fmt_err)? {
            let elt = maybe_elt.map_err(crate::fmt_err)?;
            debug_list.entry(&crate::DebugDisplay(elt));
        }
        debug_list.finish()
    }
}
