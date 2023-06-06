// @generated
#[doc(hidden)]
#[macro_export]
macro_rules! __singleton_collator_prim_v1 {
    () => {
        icu_collator::provider::CollationSpecialPrimariesV1 { last_primaries: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x06\x05\0\x0C\x8A\r\0\x0E") }, numeric_primary: 15u8 }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __lookup_collator_prim_v1 {
    ($ req : expr) => {
        $req.locale.is_empty().then(|| {
            static ANCHOR: <icu_collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::DataMarker>::Yokeable = singleton_collator_prim_v1!();
            &ANCHOR
        })
    };
}
/// Implement [`DataProvider<CollationSpecialPrimariesV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_collator_prim_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_collator::provider::CollationSpecialPrimariesV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_collator::provider::CollationSpecialPrimariesV1Marker>, icu_provider::DataError> {
                let lookup = lookup_collator_prim_v1!(req);
                lookup.map(icu_provider::prelude::zerofrom::ZeroFrom::zero_from).map(icu_provider::DataPayload::from_owned).map(|payload| icu_provider::DataResponse { metadata: Default::default(), payload: Some(payload) }).ok_or_else(|| icu_provider::DataErrorKind::MissingLocale.with_req(<icu_collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
            }
        }
    };
}