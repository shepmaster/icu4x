// generated by diplomat-tool
import { IsoDateTime } from "./IsoDateTime.mjs"
import { MetazoneCalculator } from "./MetazoneCalculator.mjs"
import { TimeZoneIdMapper } from "./TimeZoneIdMapper.mjs"
import { TimeZoneInvalidIdError } from "./TimeZoneInvalidIdError.mjs"
import { TimeZoneInvalidOffsetError } from "./TimeZoneInvalidOffsetError.mjs"
import { TimeZoneUnknownError } from "./TimeZoneUnknownError.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `CustomTimeZone`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html) for more information.
*/
const CustomTimeZone_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_CustomTimeZone_destroy_mv1(ptr);
});

export class CustomTimeZone {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("CustomTimeZone is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            CustomTimeZone_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static fromString(s) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const sSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, s));
        
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_CustomTimeZone_from_string_mv1(diplomatReceive.buffer, ...sSlice.splat());
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new TimeZoneUnknownError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('TimeZoneUnknownError', { cause });
            }
            return new CustomTimeZone(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            functionCleanupArena.free();
        
            diplomatReceive.free();
        }
    }

    static empty() {
        const result = wasm.icu4x_CustomTimeZone_empty_mv1();
    
        try {
            return new CustomTimeZone(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    static utc() {
        const result = wasm.icu4x_CustomTimeZone_utc_mv1();
    
        try {
            return new CustomTimeZone(diplomatRuntime.internalConstructor, result, []);
        }
        
        finally {}
    }

    trySetGmtOffsetSeconds(offsetSeconds) {
        const result = wasm.icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1(this.ffiValue, offsetSeconds);
    
        try {
            if (result !== 1) {
                const cause = new TimeZoneInvalidOffsetError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('TimeZoneInvalidOffsetError', { cause });
            }
    
        }
        
        finally {}
    }

    setGmtOffsetEighthsOfHour(offsetEighthsOfHour) {wasm.icu4x_CustomTimeZone_set_gmt_offset_eighths_of_hour_mv1(this.ffiValue, offsetEighthsOfHour);
    
        try {}
        
        finally {}
    }

    clearGmtOffset() {wasm.icu4x_CustomTimeZone_clear_gmt_offset_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    get gmtOffsetSeconds() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_CustomTimeZone_gmt_offset_seconds_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Int32Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0];
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    get isGmtOffsetPositive() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 2, 1, true);
        
        const result = wasm.icu4x_CustomTimeZone_is_gmt_offset_positive_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint8Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0] === 1;
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    get isGmtOffsetZero() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 2, 1, true);
        
        const result = wasm.icu4x_CustomTimeZone_is_gmt_offset_zero_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint8Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0] === 1;
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    get gmtOffsetHasMinutes() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 2, 1, true);
        
        const result = wasm.icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint8Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0] === 1;
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    get gmtOffsetHasSeconds() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 2, 1, true);
        
        const result = wasm.icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint8Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0] === 1;
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    trySetTimeZoneId(id) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const idSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, id));
        
        const result = wasm.icu4x_CustomTimeZone_try_set_time_zone_id_mv1(this.ffiValue, ...idSlice.splat());
    
        try {
            if (result !== 1) {
                const cause = new TimeZoneInvalidIdError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('TimeZoneInvalidIdError', { cause });
            }
    
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    trySetIanaTimeZoneId(mapper, id) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const idSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, id));
        
        const result = wasm.icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1(this.ffiValue, mapper.ffiValue, ...idSlice.splat());
    
        try {
            if (result !== 1) {
                const cause = new TimeZoneInvalidIdError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('TimeZoneInvalidIdError', { cause });
            }
    
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    clearTimeZoneId() {wasm.icu4x_CustomTimeZone_clear_time_zone_id_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    get timeZoneId() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_CustomTimeZone_time_zone_id_mv1(this.ffiValue, write.buffer);
    
        try {
            return result === 0 ? null : write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    trySetMetazoneId(id) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const idSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, id));
        
        const result = wasm.icu4x_CustomTimeZone_try_set_metazone_id_mv1(this.ffiValue, ...idSlice.splat());
    
        try {
            if (result !== 1) {
                const cause = new TimeZoneInvalidIdError(diplomatRuntime.internalConstructor);
                throw new globalThis.Error('TimeZoneInvalidIdError', { cause });
            }
    
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    clearMetazoneId() {wasm.icu4x_CustomTimeZone_clear_metazone_id_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    get metazoneId() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_CustomTimeZone_metazone_id_mv1(this.ffiValue, write.buffer);
    
        try {
            return result === 0 ? null : write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    trySetZoneVariant(id) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();
        
        const idSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.str8(wasm, id));
        
        const result = wasm.icu4x_CustomTimeZone_try_set_zone_variant_mv1(this.ffiValue, ...idSlice.splat());
    
        try {
            return result === 1;
        }
        
        finally {
            functionCleanupArena.free();
        }
    }

    clearZoneVariant() {wasm.icu4x_CustomTimeZone_clear_zone_variant_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    get zoneVariant() {
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        
        const result = wasm.icu4x_CustomTimeZone_zone_variant_mv1(this.ffiValue, write.buffer);
    
        try {
            return result === 0 ? null : write.readString8();
        }
        
        finally {
            write.free();
        }
    }

    setStandardTime() {wasm.icu4x_CustomTimeZone_set_standard_time_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    setDaylightTime() {wasm.icu4x_CustomTimeZone_set_daylight_time_mv1(this.ffiValue);
    
        try {}
        
        finally {}
    }

    get isStandardTime() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 2, 1, true);
        
        const result = wasm.icu4x_CustomTimeZone_is_standard_time_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint8Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0] === 1;
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    get isDaylightTime() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 2, 1, true);
        
        const result = wasm.icu4x_CustomTimeZone_is_daylight_time_mv1(diplomatReceive.buffer, this.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return (new Uint8Array(wasm.memory.buffer, diplomatReceive.buffer, 1))[0] === 1;
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    maybeCalculateMetazone(metazoneCalculator, localDatetime) {wasm.icu4x_CustomTimeZone_maybe_calculate_metazone_mv1(this.ffiValue, metazoneCalculator.ffiValue, localDatetime.ffiValue);
    
        try {}
        
        finally {}
    }
}