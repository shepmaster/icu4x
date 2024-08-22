// generated by diplomat-tool
import { DataProvider } from "./DataProvider.mjs"
import { Error } from "./Error.mjs"
import { Locale } from "./Locale.mjs"
import { TransformResult } from "./TransformResult.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** A locale expander.
*
*See the [Rust documentation for `LocaleExpander`](https://docs.rs/icu/latest/icu/locale/struct.LocaleExpander.html) for more information.
*/
const LocaleExpander_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_LocaleExpander_destroy_mv1(ptr);
});

export class LocaleExpander {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    constructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("LocaleExpander is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            LocaleExpander_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    static create(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        const result = wasm.icu4x_LocaleExpander_create_mv1(diplomatReceive.buffer, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer)) return Error[i[0]]; } return null;})();
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return new LocaleExpander(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    static createExtended(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        const result = wasm.icu4x_LocaleExpander_create_extended_mv1(diplomatReceive.buffer, provider.ffiValue);
    
        try {
            if (!diplomatReceive.resultFlag) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer)) return Error[i[0]]; } return null;})();
                throw new globalThis.Error('Error: ' + cause.value, { cause });
            }
            return new LocaleExpander(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }
        
        finally {
            diplomatReceive.free();
        }
    }

    maximize(locale) {const result = wasm.icu4x_LocaleExpander_maximize_mv1(this.ffiValue, locale.ffiValue);
    
        try {
            return TransformResult[Array.from(TransformResult.values.keys())[result]];
        }
        
        finally {}
    }

    minimize(locale) {const result = wasm.icu4x_LocaleExpander_minimize_mv1(this.ffiValue, locale.ffiValue);
    
        try {
            return TransformResult[Array.from(TransformResult.values.keys())[result]];
        }
        
        finally {}
    }

    minimizeFavorScript(locale) {const result = wasm.icu4x_LocaleExpander_minimize_favor_script_mv1(this.ffiValue, locale.ffiValue);
    
        try {
            return TransformResult[Array.from(TransformResult.values.keys())[result]];
        }
        
        finally {}
    }
}