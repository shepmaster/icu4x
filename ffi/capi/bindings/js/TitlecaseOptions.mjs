// generated by diplomat-tool
import { LeadingAdjustment } from "./LeadingAdjustment.mjs"
import { TrailingCase } from "./TrailingCase.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `TitlecaseOptions`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html) for more information.
*/
export class TitlecaseOptions {

    #leadingAdjustment;
    get leadingAdjustment()  {
        return this.#leadingAdjustment;
    }
    set leadingAdjustment(value) {
        this.#leadingAdjustment = value;
    }

    #trailingCase;
    get trailingCase()  {
        return this.#trailingCase;
    }
    set trailingCase(value) {
        this.#trailingCase = value;
    }
    constructor() {
        if (arguments.length > 0 && arguments[0] === diplomatRuntime.internalConstructor) {
            this.#fromFFI(...Array.prototype.slice.call(arguments, 1));
        } else {
            
            this.#leadingAdjustment = arguments[0];
            this.#trailingCase = arguments[1];
        }
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        return [this.#leadingAdjustment.ffiValue, this.#trailingCase.ffiValue]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    #fromFFI(ptr) {
        const leadingAdjustmentDeref = diplomatRuntime.enumDiscriminant(wasm, ptr);
        this.#leadingAdjustment = LeadingAdjustment[Array.from(LeadingAdjustment.values.keys())[leadingAdjustmentDeref]];
        const trailingCaseDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 4);
        this.#trailingCase = TrailingCase[Array.from(TrailingCase.values.keys())[trailingCaseDeref]];
    }

    static defaultOptions() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 8, 4, false);
        const result = wasm.icu4x_TitlecaseOptionsV1_default_mv1(diplomatReceive.buffer);
    
        try {
            return new TitlecaseOptions(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }
        
        finally {
            diplomatReceive.free();
        }
    }
}