// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_props_ri_v1 {
    () => {
        icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
            #[allow(unused_unsafe)]
            icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE6\xF1\x01\0\0\xF2\x01\0") }, 26usize)
        })
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_props_ri_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_properties::provider::RegionalIndicatorV1Marker as icu_provider::DataMarker>::Yokeable = singleton_props_ri_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<RegionalIndicatorV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ri_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::RegionalIndicatorV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::RegionalIndicatorV1Marker>, icu_provider::DataError> {
                let lookup = lookup_props_ri_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_properties::provider::RegionalIndicatorV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}