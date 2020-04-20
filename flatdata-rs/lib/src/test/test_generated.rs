// Do not edit: This code was generated by flatdata's generator.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(missing_docs)]
pub mod test {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const A: &str = r#"namespace test {
enum E : u32
{
    Value = 0,
}
}

namespace test {
struct A
{
    x : u32 : 16;
    y : u32 : 16;
    e : .test.E : 16;
}
}

"#;
pub const B: &str = r#"namespace test {
struct B
{
    id : u32 : 16;
}
}

"#;
pub const R: &str = r#"namespace test {
struct R
{
    @range( x )
    first_x : u32 : 16;
    y : u32 : 16;
}
}

"#;
}

pub mod s {

pub const S: &str = r#"namespace test {
enum E : u32
{
    Value = 0,
}
}

namespace test {
struct A
{
    x : u32 : 16;
    y : u32 : 16;
    e : .test.E : 16;
}
}

namespace test {
archive S
{
    data : .test.A;
}
}

"#;

pub mod resources {
pub const DATA: &str = r#"namespace test {
enum E : u32
{
    Value = 0,
}
}

namespace test {
struct A
{
    x : u32 : 16;
    y : u32 : 16;
    e : .test.E : 16;
}
}

namespace test {
archive S
{
    data : .test.A;
}
}

"#;
}
}
pub mod x {

pub const X: &str = r#"namespace test {
enum E : u32
{
    Value = 0,
}
}

namespace test {
struct A
{
    x : u32 : 16;
    y : u32 : 16;
    e : .test.E : 16;
}
}

namespace test {
archive X
{
    data : vector< .test.A >;
}
}

"#;

pub mod resources {
pub const DATA: &str = r#"namespace test {
enum E : u32
{
    Value = 0,
}
}

namespace test {
struct A
{
    x : u32 : 16;
    y : u32 : 16;
    e : .test.E : 16;
}
}

namespace test {
archive X
{
    data : vector< .test.A >;
}
}

"#;
}
}
pub mod y {

pub const Y: &str = r#"namespace test {
struct R
{
    @range( x )
    first_x : u32 : 16;
    y : u32 : 16;
}
}

namespace test {
archive Y
{
    data : vector< .test.R >;
}
}

"#;

pub mod resources {
pub const DATA: &str = r#"namespace test {
struct R
{
    @range( x )
    first_x : u32 : 16;
    y : u32 : 16;
}
}

namespace test {
archive Y
{
    data : vector< .test.R >;
}
}

"#;
}
}
pub mod z {

pub const Z: &str = r#"namespace test {
enum E : u32
{
    Value = 0,
}
}

namespace test {
struct A
{
    x : u32 : 16;
    y : u32 : 16;
    e : .test.E : 16;
}
}

namespace test {
struct B
{
    id : u32 : 16;
}
}

namespace test {
archive Z
{
    ab : multivector< 16, .test.A, .test.B >;
}
}

"#;

pub mod resources {
pub const AB: &str = r#"namespace test {
enum E : u32
{
    Value = 0,
}
}

namespace test {
struct A
{
    x : u32 : 16;
    y : u32 : 16;
    e : .test.E : 16;
}
}

namespace test {
struct B
{
    id : u32 : 16;
}
}

namespace test {
archive Z
{
    ab : multivector< 16, .test.A, .test.B >;
}
}

"#;
}
}
pub mod w {

pub const W: &str = r#"namespace test {
archive W
{
    blob : raw_data;
}
}

"#;

pub mod resources {
pub const BLOB: &str = r#"namespace test {
archive W
{
    blob : raw_data;
}
}

"#;
}
}
}#[repr(transparent)]
#[derive(Clone)]
pub struct A {
    data: [u8; 6],
}

impl A {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 6]}
    }
}

impl crate::Struct for A {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 6]}
    }

    const SCHEMA: &'static str = schema::structs::A;
    const SIZE_IN_BYTES: usize = 6;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl A {
    pub fn new( ) -> Self {
        Self{data : [0; 6]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 6]) -> &Self {
        // Safety: This is safe since A is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 6]) -> &mut Self {
        // Safety: This is safe since A is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, crate::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 6 {
            assert_eq!(data.len(), 6);
            return Err(crate::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 6];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, crate::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 6 {
            assert_eq!(data.len(), 6);
            return Err(crate::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 6];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 6] {
        &self.data
    }
}

impl Default for A {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl crate::NoOverlap for A {}

impl A {
    #[inline]
    pub fn x(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 16);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[inline]
    pub fn y(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 16, 16);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[inline]
    pub fn e(&self) -> super::test::E {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 32, 16);
        unsafe { std::mem::transmute::<u32, super::test::E>(value) }
    }

}

impl std::fmt::Debug for A {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("A")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("e", &self.e())
            .finish()
    }
}

impl std::cmp::PartialEq for A {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() &&        self.y() == other.y() &&        self.e() == other.e()     }
}

impl A {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_x(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 16)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn set_y(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 16, 16)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn set_e(&mut self, value: super::test::E) {
        flatdata_write_bytes!(u32; value, self.data, 32, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &A) {
        self.set_x(other.x());
        self.set_y(other.y());
        self.set_e(other.e());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct B {
    data: [u8; 2],
}

impl B {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }
}

impl crate::Struct for B {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }

    const SCHEMA: &'static str = schema::structs::B;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl B {
    pub fn new( ) -> Self {
        Self{data : [0; 2]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 2]) -> &Self {
        // Safety: This is safe since B is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 2]) -> &mut Self {
        // Safety: This is safe since B is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, crate::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 2 {
            assert_eq!(data.len(), 2);
            return Err(crate::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 2];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, crate::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 2 {
            assert_eq!(data.len(), 2);
            return Err(crate::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 2];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 2] {
        &self.data
    }
}

impl Default for B {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl crate::NoOverlap for B {}

impl B {
    #[inline]
    pub fn id(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 16);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl std::fmt::Debug for B {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("B")
            .field("id", &self.id())
            .finish()
    }
}

impl std::cmp::PartialEq for B {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()     }
}

impl B {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_id(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &B) {
        self.set_id(other.id());
    }
}
#[repr(transparent)]
pub struct R {
    data: [u8; 4],
}

impl R {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl crate::Struct for R {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::R;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;
}


impl R {
    /// First element of the range [`x`].
    ///
    /// [`x`]: #method.x
    #[inline]
    pub fn first_x(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 16);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[inline]
    pub fn x(&self) -> std::ops::Range<u32> {
        let start = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 16);
        let end = flatdata_read_bytes!(u32, self.data.as_ptr(), 0 + 4 * 8, 16);
        start..end
    }

    #[inline]
    pub fn y(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 16, 16);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl std::fmt::Debug for R {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("R")
            .field("first_x", &self.first_x())
            .field("y", &self.y())
            .finish()
    }
}

impl std::cmp::PartialEq for R {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.first_x() == other.first_x() &&        self.y() == other.y()     }
}

impl R {
    /// First element of the range [`x`].
    ///
    /// [`x`]: struct.RRef.html#method.x
    #[inline]
    #[allow(missing_docs)]
    pub fn set_first_x(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 16)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn set_y(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 16, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &R) {
        self.set_first_x(other.first_x());
        self.set_y(other.y());
    }
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum E {
    Value = 0,
}

impl crate::helper::Int for E {
    const IS_SIGNED: bool = false;
}



#[derive(Clone)]
pub struct S {
    _storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    data : &'static super::test::A,
}

impl S {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &super::test::A {
        self.data
    }

}

impl ::std::fmt::Debug for S {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("S")
            .field("data", &self.data())
            .finish()
    }
}

impl crate::Archive for S {
    const NAME: &'static str = "S";
    const SCHEMA: &'static str = schema::s::S;

    fn open(storage: ::std::rc::Rc<dyn crate::ResourceStorage>)
        -> ::std::result::Result<Self, crate::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use crate::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], crate::ResourceStorageError>| -> Result<&'static [u8], crate::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("data", schema::s::resources::DATA));
        let data = resource.map(|x| super::test::A::from_bytes_slice(x))??;

        Ok(Self {
            _storage: storage,
            data,
        })
    }
}

/// Builder for creating [`S`] archives.
///
///[`S`]: struct.S.html
#[derive(Clone, Debug)]
pub struct SBuilder {
    storage: ::std::rc::Rc<dyn crate::ResourceStorage>
}

impl SBuilder {
    #[inline]
    /// Stores [`data`] in the archive.
    ///
    /// [`data`]: struct.S.html#method.data
    /// Stores [`data`] in the archive.
    pub fn set_data(&self, resource: &super::test::A) -> ::std::io::Result<()> {
        let data = resource.as_bytes();
        self.storage.write("data", schema::s::resources::DATA, data)
    }

}

impl crate::ArchiveBuilder for SBuilder {
    const NAME: &'static str = "S";
    const SCHEMA: &'static str = schema::s::S;

    fn new(
        storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    ) -> Result<Self, crate::ResourceStorageError> {
        crate::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}




#[derive(Clone)]
pub struct X {
    _storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    data : &'static [super::test::A],
}

impl X {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &[super::test::A] {
        self.data
    }

}

impl ::std::fmt::Debug for X {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("X")
            .field("data", &self.data())
            .finish()
    }
}

impl crate::Archive for X {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn open(storage: ::std::rc::Rc<dyn crate::ResourceStorage>)
        -> ::std::result::Result<Self, crate::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use crate::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], crate::ResourceStorageError>| -> Result<&'static [u8], crate::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("data", schema::x::resources::DATA));
        let data = resource.map(|x| <&[super::test::A]>::from_bytes(x))??;

        Ok(Self {
            _storage: storage,
            data,
        })
    }
}

/// Builder for creating [`X`] archives.
///
///[`X`]: struct.X.html
#[derive(Clone, Debug)]
pub struct XBuilder {
    storage: ::std::rc::Rc<dyn crate::ResourceStorage>
}

impl XBuilder {
    #[inline]
    /// Stores [`data`] in the archive.
    ///
    /// [`data`]: struct.X.html#method.data
    pub fn set_data(&self, vector: &[super::test::A]) -> ::std::io::Result<()> {
        use crate::SliceExt;
        self.storage.write("data", schema::x::resources::DATA, vector.as_bytes())
    }

    /// Opens [`data`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`data`]: struct.X.html#method.data
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_data(&self) -> ::std::io::Result<crate::ExternalVector<super::test::A>> {
        crate::create_external_vector(&*self.storage, "data", schema::x::resources::DATA)
    }

}

impl crate::ArchiveBuilder for XBuilder {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn new(
        storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    ) -> Result<Self, crate::ResourceStorageError> {
        crate::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}




#[derive(Clone)]
pub struct Y {
    _storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    data : &'static [super::test::R],
}

impl Y {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &[super::test::R] {
        self.data
    }

}

impl ::std::fmt::Debug for Y {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Y")
            .field("data", &self.data())
            .finish()
    }
}

impl crate::Archive for Y {
    const NAME: &'static str = "Y";
    const SCHEMA: &'static str = schema::y::Y;

    fn open(storage: ::std::rc::Rc<dyn crate::ResourceStorage>)
        -> ::std::result::Result<Self, crate::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use crate::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], crate::ResourceStorageError>| -> Result<&'static [u8], crate::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("data", schema::y::resources::DATA));
        let data = resource.map(|x| <&[super::test::R]>::from_bytes(x))??;

        Ok(Self {
            _storage: storage,
            data,
        })
    }
}

/// Builder for creating [`Y`] archives.
///
///[`Y`]: struct.Y.html
#[derive(Clone, Debug)]
pub struct YBuilder {
    storage: ::std::rc::Rc<dyn crate::ResourceStorage>
}

impl YBuilder {
    #[inline]
    /// Stores [`data`] in the archive.
    ///
    /// [`data`]: struct.Y.html#method.data
    pub fn set_data(&self, vector: &[super::test::R]) -> ::std::io::Result<()> {
        use crate::SliceExt;
        self.storage.write("data", schema::y::resources::DATA, vector.as_bytes())
    }

    /// Opens [`data`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`data`]: struct.Y.html#method.data
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_data(&self) -> ::std::io::Result<crate::ExternalVector<super::test::R>> {
        crate::create_external_vector(&*self.storage, "data", schema::y::resources::DATA)
    }

}

impl crate::ArchiveBuilder for YBuilder {
    const NAME: &'static str = "Y";
    const SCHEMA: &'static str = schema::y::Y;

    fn new(
        storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    ) -> Result<Self, crate::ResourceStorageError> {
        crate::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}



/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`ab`] resource.
///
/// [`ab`]: struct.Archive{.test.Z}.html#method.ab
#[derive(Clone, PartialEq)]
pub enum AbRef<'a> {
    #[allow(missing_docs)]
    A(&'a super::test::A),    #[allow(missing_docs)]
    B(&'a super::test::B),}

impl<'a> ::std::fmt::Debug for AbRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AbRef::A(ref inner) => write!(f, "{:?}", inner),
            AbRef::B(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> crate::VariadicRef for AbRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            AbRef::A(_) => <super::test::A as crate::Struct>::SIZE_IN_BYTES,
            AbRef::B(_) => <super::test::B as crate::Struct>::SIZE_IN_BYTES,
        }
    }
}

/// Builder of buckets in the [`ab`] resource.
///
/// Refers to a single bucket in the [`ab`] multivector and
/// provides methods for adding heterogeneous data to the bucket.
///
/// [`ab`]: struct.Archive{.test.Z}.html#method.ab
pub struct AbBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> AbBuilder<'a> {
    /// Adds data of the type [`A`] to the bucket.
    ///
    /// [`A`]: struct.A.html
    #[inline]
    pub fn add_a<'b>(&'b mut self) -> &'b mut super::test::A {
        let old_len = self.data.len();
        let increment = 1 + <super::test::A as crate::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 0;
        let slice = &mut self.data[1 + old_len..];
        super::test::A::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::test::A from slice")
    }
    /// Adds data of the type [`B`] to the bucket.
    ///
    /// [`B`]: struct.B.html
    #[inline]
    pub fn add_b<'b>(&'b mut self) -> &'b mut super::test::B {
        let old_len = self.data.len();
        let increment = 1 + <super::test::B as crate::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 1;
        let slice = &mut self.data[1 + old_len..];
        super::test::B::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::test::B from slice")
    }
}

/// Variadic struct attached to the [`ab`] archive resource.
///
/// It unifies the following data types:
//
/// * [`A`]
/// * [`B`]
///
/// ## Access pattern
///
/// This structure is used as a template parameter in [`ab`] multivector/
/// multiarray view. It does not contain any data, instead it references
///
/// * [`AbRef`] for the read-only heterogeneous access, and
/// * [`AbBuilder`] for the mutable builder pattern access.
///
/// [`ab`]: struct.Archive{.test.Z}.html#method.ab
/// [`AbRef`]: enum.AbRef.html
/// [`AbBuilder`]: struct.AbBuilder.html
/// [`A`]: struct.A.html
/// [`B`]: struct.B.html
#[derive(Clone)]
pub struct Ab {}

impl crate::VariadicIndex for Ab {
    type Index = super::_builtin::multivector::IndexType16;
}

impl<'a> crate::VariadicStruct<'a> for Ab {
    type Item = AbRef<'a>;

    #[inline]
    fn create(index: crate::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => AbRef::A(super::test::A::from_bytes_slice(&data).expect("Corrupted data")),
                1 => AbRef::B(super::test::B::from_bytes_slice(&data).expect("Corrupted data")),
            _ => panic!("invalid type index {} for variadic type AbRef", index),
        }
    }

    type ItemMut = AbBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}

#[derive(Clone)]
pub struct Z {
    _storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    ab : crate::MultiArrayView<'static, Ab>,
}

impl Z {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn ab(&self) -> &crate::MultiArrayView<Ab> {
        &self.ab
    }

}

impl ::std::fmt::Debug for Z {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("Z")
            .field("ab", &self.ab())
            .finish()
    }
}

impl crate::Archive for Z {
    const NAME: &'static str = "Z";
    const SCHEMA: &'static str = schema::z::Z;

    fn open(storage: ::std::rc::Rc<dyn crate::ResourceStorage>)
        -> ::std::result::Result<Self, crate::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use crate::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], crate::ResourceStorageError>| -> Result<&'static [u8], crate::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let ab = {
            let index_schema = &format!("index({})", schema::z::resources::AB);
            let index = extend(storage.read("ab_index", &index_schema));
            let data = extend(storage.read("ab", schema::z::resources::AB));
            let result = match (index, data) {
                (Ok(index), Ok(data)) => {
                    Ok(crate::MultiArrayView::new(
                        <&[super::_builtin::multivector::IndexType16]>::from_bytes(index)?,
                        data
                    ))
                }
                (Ok(_), Err(x)) | (Err(x), Ok(_)) => {return Err(x);}
                (Err(x), Err(_)) => Err(x),
            };
            result?
        };

        Ok(Self {
            _storage: storage,
            ab,
        })
    }
}

/// Builder for creating [`Z`] archives.
///
///[`Z`]: struct.Z.html
#[derive(Clone, Debug)]
pub struct ZBuilder {
    storage: ::std::rc::Rc<dyn crate::ResourceStorage>
}

impl ZBuilder {
    /// Opens [`ab`] in the archive for buffered writing.
    ///
    /// Elements can be added to the multivector until the [`MultiVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`ab`]: struct.Z.html#method.ab
    /// [`MultiVector::close`]: flatdata/struct.MultiVector.html#method.close
    #[inline]
    pub fn start_ab(&self) -> ::std::io::Result<crate::MultiVector<Ab>> {
        crate::create_multi_vector(&*self.storage, "ab", schema::z::resources::AB)
    }

}

impl crate::ArchiveBuilder for ZBuilder {
    const NAME: &'static str = "Z";
    const SCHEMA: &'static str = schema::z::Z;

    fn new(
        storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    ) -> Result<Self, crate::ResourceStorageError> {
        crate::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}




#[derive(Clone)]
pub struct W {
    _storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    blob : crate::RawData<'static>,
}

impl W {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn blob(&self) -> crate::RawData {
        self.blob
    }

}

impl ::std::fmt::Debug for W {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("W")
            .field("blob", &self.blob())
            .finish()
    }
}

impl crate::Archive for W {
    const NAME: &'static str = "W";
    const SCHEMA: &'static str = schema::w::W;

    fn open(storage: ::std::rc::Rc<dyn crate::ResourceStorage>)
        -> ::std::result::Result<Self, crate::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use crate::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], crate::ResourceStorageError>| -> Result<&'static [u8], crate::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("blob", schema::w::resources::BLOB));
        let blob = resource.map(|x| crate::RawData::new(x))?;

        Ok(Self {
            _storage: storage,
            blob,
        })
    }
}

/// Builder for creating [`W`] archives.
///
///[`W`]: struct.W.html
#[derive(Clone, Debug)]
pub struct WBuilder {
    storage: ::std::rc::Rc<dyn crate::ResourceStorage>
}

impl WBuilder {
    /// Stores [`blob`] in the archive.
    ///
    /// [`blob`]: struct.W.html#method.blob
    #[inline]
    pub fn set_blob(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("blob", schema::w::resources::BLOB, data)
    }

}

impl crate::ArchiveBuilder for WBuilder {
    const NAME: &'static str = "W";
    const SCHEMA: &'static str = schema::w::W;

    fn new(
        storage: ::std::rc::Rc<dyn crate::ResourceStorage>,
    ) -> Result<Self, crate::ResourceStorageError> {
        crate::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}

}

#[doc(hidden)]
pub mod _builtin {

#[allow(missing_docs)]
pub mod multivector {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const INDEX_TYPE16: &str = r#""#;
}

}
/// Builtin type to for MultiVector index
#[repr(transparent)]
pub struct IndexType16 {
    data: [u8; 2],
}

impl IndexType16 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }
}

impl crate::Struct for IndexType16 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }

    const SCHEMA: &'static str = schema::structs::INDEX_TYPE16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;
}


impl IndexType16 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 16);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 16);
        let end = flatdata_read_bytes!(u64, self.data.as_ptr(), 0 + 2 * 8, 16);
        start..end
    }

}

impl std::fmt::Debug for IndexType16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType16")
            .field("value", &self.value())
            .finish()
    }
}

impl std::cmp::PartialEq for IndexType16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl IndexType16 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType16Ref.html#method.range
    #[inline]
    #[allow(missing_docs)]
    pub fn set_value(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType16) {
        self.set_value(other.value());
    }
}

impl crate::IndexStruct for IndexType16 {
    #[inline]
    fn range(&self) -> std::ops::Range<usize> {
        let range = self.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(&mut self, value: usize) {
        self.set_value(value as u64);
    }
}

}

#[doc(hidden)]
pub mod schema {
pub mod structs {
}

}}