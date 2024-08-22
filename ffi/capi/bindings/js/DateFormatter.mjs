// generated by diplomat-tool
import { DataProvider } from "./DataProvider.mjs"
import { Date } from "./Date.mjs"
import { DateLength } from "./DateLength.mjs"
import { DateTime } from "./DateTime.mjs"
import { Error } from "./Error.mjs"
import { IsoDate } from "./IsoDate.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { Locale } from "./Locale.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X DateFormatter object capable of formatting a [`Date`] as a string,
*using some calendar specified at runtime in the locale.
*
*See the [Rust documentation for `DateFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html) for more information.
*/
const DateFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_DateFormatter_destroy_mv1(ptr);
});

export class DateFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("DateFormatter is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            DateFormatter_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static createWithLength(provider, locale, dateLength) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        const result = wasm.icu4x_DateFormatter_create_with_length_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue, dateLength.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer)) return Error[i[0]]; } return null;})();
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return new DateFormatter(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    formatDate(value) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        const result = wasm.icu4x_DateFormatter_format_date_mv1(diplomatReceive.buffer, this.ffiValue, value.ffiValue, write.buffer);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer)) return Error[i[0]]; } return null;})();
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return write.readString8();
        }
        
        finally {
            diplomatReceive.free();
        
            write.free();
        }
    }

    formatIsoDate(value) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        const result = wasm.icu4x_DateFormatter_format_iso_date_mv1(diplomatReceive.buffer, this.ffiValue, value.ffiValue, write.buffer);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer)) return Error[i[0]]; } return null;})();
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return write.readString8();
        }
        
        finally {
            diplomatReceive.free();
        
            write.free();
        }
    }

    formatDatetime(value) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        const result = wasm.icu4x_DateFormatter_format_datetime_mv1(diplomatReceive.buffer, this.ffiValue, value.ffiValue, write.buffer);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer)) return Error[i[0]]; } return null;})();
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return write.readString8();
        }
        
        finally {
            diplomatReceive.free();
        
            write.free();
        }
    }

    formatIsoDatetime(value) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const write = new diplomatRuntime.DiplomatWriteBuf(wasm);
        const result = wasm.icu4x_DateFormatter_format_iso_datetime_mv1(diplomatReceive.buffer, this.ffiValue, value.ffiValue, write.buffer);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer)) return Error[i[0]]; } return null;})();
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return write.readString8();
        }
        
        finally {
            diplomatReceive.free();
        
            write.free();
        }
    }
}