// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `LineBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html) for more information.
*
*Additional information: [1](https://docs.rs/icu/latest/icu/segmenter/type.LineBreakIteratorPotentiallyIllFormedUtf8.html)
*/
const LineBreakIteratorUtf8_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_LineBreakIteratorUtf8_destroy_mv1(ptr);
});

export class LineBreakIteratorUtf8 {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    #aEdge = [];
    
    constructor(symbol, ptr, selfEdge, aEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("LineBreakIteratorUtf8 is an Opaque type. You cannot call its constructor.");
            return;
        }
        
        
        this.#aEdge = aEdge;
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        
        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            LineBreakIteratorUtf8_box_destroy_registry.register(this, this.#ptr);
        }
    }

    get ffiValue() {
        return this.#ptr;
    }

    next() {const result = wasm.icu4x_LineBreakIteratorUtf8_next_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }
}