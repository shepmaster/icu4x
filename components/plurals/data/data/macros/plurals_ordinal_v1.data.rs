// @generated
/// Implement [`DataProvider<OrdinalV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_plurals_ordinal_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_plurals::provider::OrdinalV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_plurals::provider::OrdinalV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    static AF: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: None, many: None };
                    static IT: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: None, many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x0B\0\0\0\x0B\0\0\0\x08\0\0\0\x08\0\0\0P\0\0\0P\0\0\0 \x03\0\0 \x03\0\0") })) };
                    static KK: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: None, many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x04\0\0\0\0\0\r\0\x1A\0'\0\xC0\n\0\0\0\x06\0\0\0\x06\0\0\0@\n\0\0\0\t\0\0\0\t\0\0\0@\n\0\0\0\0\0\0\0\0\0\0\0\x80\0\0\0\0\0\0\0\0\0\0\0\0") })) };
                    static UK: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x03\0\0\0\x03\0\0\0\x80d\0\0\0\r\0\0\0\r\0\0\0") })), many: None };
                    static BE: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xC0\n\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0\x80d\0\0\0\x0C\0\0\0\x0C\0\0\0\r\0\0\0\r\0\0\0") })), many: None };
                    static TK: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: None, two: None, few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xC0\n\0\0\0\x06\0\0\0\x06\0\0\0\t\0\0\0\t\0\0\0@\0\0\0\0\n\0\0\0\n\0\0\0") })), many: None };
                    static FIL: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: None };
                    static SQ: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x04\0\0\0\x04\0\0\0\x80d\0\0\0\x0E\0\0\0\x0E\0\0\0") })) };
                    static MR: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x04\0\0\0\x04\0\0\0") })), many: None };
                    static GU: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x04\0\0\0\x04\0\0\0") })), many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x06\0\0\0\x06\0\0\0") })) };
                    static CA: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x03\0\0\0\x03\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x04\0\0\0\x04\0\0\0") })), many: None };
                    static HU: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x05\0\0\0\x05\0\0\0") })), two: None, few: None, many: None };
                    static OR: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x05\0\0\0\x05\0\0\0\x07\0\0\0\t\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x04\0\0\0\x04\0\0\0") })), many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x06\0\0\0\x06\0\0\0") })) };
                    static AS: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x05\0\0\0\x05\0\0\0\x07\0\0\0\x07\0\0\0\x08\0\0\0\x08\0\0\0\t\0\0\0\t\0\0\0\n\0\0\0\n\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x03\0\0\0\x03\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x04\0\0\0\x04\0\0\0") })), many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x06\0\0\0\x06\0\0\0") })) };
                    static GD: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0\x0C\0\0\0\x0C\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x03\0\0\0\x03\0\0\0\r\0\0\0\r\0\0\0") })), many: None };
                    static NE: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x04\0\0\0") })), two: None, few: None, many: None };
                    static KA: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC1\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: None, few: None, many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\0\0\0\0\0\0\0\0\0\0\0\0Ad\0\0\0\x02\0\0\0\x14\0\0\0(\0\0\0(\0\0\0<\0\0\0<\0\0\0P\0\0\0P\0\0\0") })) };
                    static AZ: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0-\0\xC1\n\0\0\0\x01\0\0\0\x01\0\0\0\x02\0\0\0\x02\0\0\0\x05\0\0\0\x05\0\0\0\x07\0\0\0\x07\0\0\0\x08\0\0\0\x08\0\0\0Ad\0\0\0\x14\0\0\0\x14\0\0\x002\0\0\x002\0\0\0F\0\0\0F\0\0\0P\0\0\0P\0\0\0") })), two: None, few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xC1\n\0\0\0\x03\0\0\0\x03\0\0\0\x04\0\0\0\x04\0\0\0A\xE8\x03\0\0d\0\0\0d\0\0\0\xC8\0\0\0\xC8\0\0\0,\x01\0\0,\x01\0\0\x90\x01\0\0\x90\x01\0\0\xF4\x01\0\0\xF4\x01\0\0X\x02\0\0X\x02\0\0\xBC\x02\0\0\xBC\x02\0\0 \x03\0\0 \x03\0\0\x84\x03\0\0\x84\x03\0\0") })), many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x03\0\0\0\0\0\r\0\x1A\0\xC1\0\0\0\0\0\0\0\0\0\0\0\0A\n\0\0\0\x06\0\0\0\x06\0\0\0Ad\0\0\0(\0\0\0(\0\0\0<\0\0\0<\0\0\0Z\0\0\0Z\0\0\0") })) };
                    static EN: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x80d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x02\0\0\0\x02\0\0\0\x80d\0\0\0\x0C\0\0\0\x0C\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC0\n\0\0\0\x03\0\0\0\x03\0\0\0\x80d\0\0\0\r\0\0\0\r\0\0\0") })), many: None };
                    static MK: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\n\0\0\0\x01\0\0\0\x01\0\0\0\x81d\0\0\0\x0B\0\0\0\x0B\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xC1\n\0\0\0\x02\0\0\0\x02\0\0\0\x81d\0\0\0\x0C\0\0\0\x0C\0\0\0") })), few: None, many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xC1\n\0\0\0\x07\0\0\0\x07\0\0\0\x08\0\0\0\x08\0\0\0\x81d\0\0\0\x11\0\0\0\x11\0\0\0\x12\0\0\0\x12\0\0\0") })) };
                    static SV: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: None, one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0\xC0\n\0\0\0\x01\0\0\0\x01\0\0\0\x02\0\0\0\x02\0\0\0\x80d\0\0\0\x0B\0\0\0\x0B\0\0\0\x0C\0\0\0\x0C\0\0\0") })), two: None, few: None, many: None };
                    static CY: <icu_plurals::provider::OrdinalV1Marker as icu_provider::DataMarker>::Yokeable = icu_plurals::provider::PluralRulesV1 { zero: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\x07\0\0\0\x08\0\0\0\x08\0\0\0\t\0\0\0\t\0\0\0") })), one: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x01\0\0\0\x01\0\0\0") })), two: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x02\0\0\0\x02\0\0\0") })), few: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x03\0\0\0\x03\0\0\0\x04\0\0\0\x04\0\0\0") })), many: Some(icu_plurals::rules::runtime::ast::Rule(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x01\0\0\0\0\0\xC0\0\0\0\0\x05\0\0\0\x05\0\0\0\x06\0\0\0\x06\0\0\0") })) };
                    match ["af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "cs", "cy", "da", "de", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "ga", "gd", "gl", "gu", "he", "hi", "hr", "hu", "hy", "id", "is", "it", "ja", "ka", "kk", "km", "kn", "ko", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mr", "ms", "my", "ne", "nl", "no", "or", "pa", "pl", "ps", "pt", "ro", "ru", "sd", "si", "sk", "sl", "sq", "sr", "sv", "sw", "ta", "te", "th", "tk", "tr", "uk", "ur", "uz", "vi", "yue", "zh", "zu"].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                        Ok(i) => Ok(*unsafe { [&AF, &AF, &AF, &AS, &AZ, &BE, &AF, &AS, &AF, &CA, &AF, &CY, &AF, &AF, &AF, &EN, &AF, &AF, &AF, &AF, &AF, &FIL, &FIL, &FIL, &GD, &AF, &GU, &AF, &GU, &AF, &HU, &FIL, &AF, &AF, &IT, &AF, &KA, &KK, &AF, &AF, &AF, &AF, &FIL, &AF, &AF, &MK, &AF, &AF, &MR, &FIL, &AF, &NE, &AF, &AF, &OR, &AF, &AF, &AF, &AF, &FIL, &AF, &AF, &AF, &AF, &AF, &SQ, &AF, &SV, &AF, &AF, &AF, &AF, &TK, &AF, &UK, &AF, &AF, &FIL, &AF, &AF, &AF].get_unchecked(i) }),
                        Err(_) => Err(icu_provider::DataErrorKind::MissingLocale),
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_plurals::provider::OrdinalV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}