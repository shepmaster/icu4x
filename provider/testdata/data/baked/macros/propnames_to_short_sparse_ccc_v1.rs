// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_propnames_to_short_sparse_ccc_v1 {
    () => {
        icu_properties::provider::names::PropertyEnumToValueNameSparseMapV1 {
            map: unsafe {
                #[allow(unused_unsafe)]
                zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\x01\0\x06\0\x07\0\x08\0\t\0\n\0\x0B\0\x0C\0\r\0\x0E\0\x0F\0\x10\0\x11\0\x12\0\x13\0\x14\0\x15\0\x16\0\x17\0\x18\0\x19\0\x1A\0\x1B\0\x1C\0\x1D\0\x1E\0\x1F\0 \0!\0\"\0#\0$\0T\0[\0g\0k\0v\0z\0\x81\0\x82\0\x84\0\x85\0\xC8\0\xCA\0\xD6\0\xD8\0\xDA\0\xDC\0\xDE\0\xE0\0\xE2\0\xE4\0\xE6\0\xE8\0\xE9\0\xEA\0\xF0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b":\0\0\0\0\0\x02\0\x04\0\x08\0\n\0\x0C\0\x0E\0\x13\0\x18\0\x1D\0\"\0'\0,\x001\x006\0;\0@\0E\0J\0O\0T\0Y\0^\0c\0h\0m\0r\0w\0|\0\x81\0\x86\0\x8B\0\x90\0\x95\0\x9A\0\x9F\0\xA5\0\xAB\0\xB1\0\xB7\0\xBD\0\xC3\0\xC9\0\xCF\0\xD3\0\xD6\0\xD9\0\xDD\0\xDF\0\xE0\0\xE2\0\xE3\0\xE4\0\xE6\0\xE7\0\xE9\0\xEB\0\xED\0NROVHANRNKKVVRCCC10CCC11CCC12CCC13CCC14CCC15CCC16CCC17CCC18CCC19CCC20CCC21CCC22CCC23CCC24CCC25CCC26CCC27CCC28CCC29CCC30CCC31CCC32CCC33CCC34CCC35CCC36CCC84CCC91CCC103CCC107CCC118CCC122CCC129CCC130CCC132CCC133ATBLATBATAATARBLBBRLRALAARDBDAIS") })
            },
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_propnames_to_short_sparse_ccc_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable = singleton_propnames_to_short_sparse_ccc_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<CanonicalCombiningClassValueToShortNameV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_short_sparse_ccc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker>, icu_provider::DataError> {
                let lookup = lookup_propnames_to_short_sparse_ccc_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::CanonicalCombiningClassValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}