// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_uideo_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x004\0\0\xC0M\0\0\0N\0\0\0\xA0\0\0\x0E\xFA\0\0\x10\xFA\0\0\x11\xFA\0\0\x12\xFA\0\0\x13\xFA\0\0\x15\xFA\0\0\x1F\xFA\0\0 \xFA\0\0!\xFA\0\0\"\xFA\0\0#\xFA\0\0%\xFA\0\0'\xFA\0\0*\xFA\0\0\0\0\x02\0\xE0\xA6\x02\0\0\xA7\x02\0:\xB7\x02\0@\xB7\x02\0\x1E\xB8\x02\0 \xB8\x02\0\xA2\xCE\x02\0\xB0\xCE\x02\0\xE1\xEB\x02\0\0\0\x03\0K\x13\x03\0P\x13\x03\0\xB0#\x03\0") }, 97058usize)
        })
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_props_uideo_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_properties::provider::UnifiedIdeographV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_uideo_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<UnifiedIdeographV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_uideo_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::UnifiedIdeographV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::UnifiedIdeographV1Marker>, icu_provider::DataError> {
                let lookup = lookup_props_uideo_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::UnifiedIdeographV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}