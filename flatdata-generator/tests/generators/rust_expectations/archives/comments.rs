// This is a comment about foo
#[derive(Clone)]
pub struct Foo {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    bar : flatdata::RawData<'static>,
}

impl Foo {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    // this is a comment about foo.bar
    #[inline]
    pub fn bar(&self) -> flatdata::RawData {
        self.bar
    }

}

impl ::std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Foo")
            .field("bar", &self.bar())
            .finish()
    }
}

impl flatdata::Archive for Foo {
    const NAME: &'static str = "Foo";
    const SCHEMA: &'static str = schema::foo::FOO;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use flatdata::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], flatdata::ResourceStorageError>| -> Result<&'static [u8], flatdata::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("bar", schema::foo::resources::BAR));
        let bar = resource.map(|x| flatdata::RawData::new(x))?;

        Ok(Self {
            _storage: storage,
            bar,
        })
    }
}

/// Builder for creating [`Foo`] archives.
///
///[`Foo`]: struct.Foo.html
#[derive(Clone, Debug)]
pub struct FooBuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl FooBuilder {
    /// Stores [`bar`] in the archive.
    ///
    /// [`bar`]: struct.Foo.html#method.bar
    #[inline]
    pub fn set_bar(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("bar", schema::foo::resources::BAR, data)
    }

}

impl flatdata::ArchiveBuilder for FooBuilder {
    const NAME: &'static str = "Foo";
    const SCHEMA: &'static str = schema::foo::FOO;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}




/// This is a comment about Bar
#[derive(Clone)]
pub struct Bar {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    foo : flatdata::RawData<'static>,
}

impl Bar {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    /// this is a comment about bar.foo
    #[inline]
    pub fn foo(&self) -> flatdata::RawData {
        self.foo
    }

}

impl ::std::fmt::Debug for Bar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Bar")
            .field("foo", &self.foo())
            .finish()
    }
}

impl flatdata::Archive for Bar {
    const NAME: &'static str = "Bar";
    const SCHEMA: &'static str = schema::bar::BAR;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use flatdata::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], flatdata::ResourceStorageError>| -> Result<&'static [u8], flatdata::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("foo", schema::bar::resources::FOO));
        let foo = resource.map(|x| flatdata::RawData::new(x))?;

        Ok(Self {
            _storage: storage,
            foo,
        })
    }
}

/// Builder for creating [`Bar`] archives.
///
///[`Bar`]: struct.Bar.html
#[derive(Clone, Debug)]
pub struct BarBuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl BarBuilder {
    /// Stores [`foo`] in the archive.
    ///
    /// [`foo`]: struct.Bar.html#method.foo
    #[inline]
    pub fn set_foo(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("foo", schema::bar::resources::FOO, data)
    }

}

impl flatdata::ArchiveBuilder for BarBuilder {
    const NAME: &'static str = "Bar";
    const SCHEMA: &'static str = schema::bar::BAR;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}

}
