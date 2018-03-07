/// This is a weapon of mass unsafety. Don't use that. Please. Just don't.
#[macro_export]
macro_rules! const_uninitialized {
    ($t: ty) => {{
        union TerrifyingUnion {
            data: $t,
            junk: u8
        }
        const JUNK: TerrifyingUnion = TerrifyingUnion { junk: 0 };
        unsafe { JUNK.data }
    }}
}
