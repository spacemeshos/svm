'use strict';

const fs = require('fs');

async function compileWasmCodec () {
    const wasm = await WebAssembly.compile(fs.readFileSync('../../../target/wasm32-unknown-unknown/debug/svm_codec.wasm'));
    const importObject = {};

    return WebAssembly.instantiate(wasm, importObject)
}

const assert = require('assert');

describe('Allocate/Free WASM Buffer', function () {
    it('Allocate WASM buffer', function () {
	return compileWasmCodec().then(instance => {
	    // 1) allocating 100 bytes WASM buffer
	    const buf = instance.exports.wasm_alloc(100);

	    // 2) copying data to WASM buffer
	    // asserting the allocated WASM buffer `Data` length
	    let data_len = instance.exports.wasm_buffer_length(buf);
	    assert.equal(data_len, 100);

	    // extracting the pointer to the `Data` section
	    let data = instance.exports.wasm_buffer_data(buf);
	    let memory = instance.exports.memory.buffer;
	    let view = new Uint8Array(memory);

	    view.set([10, 20, 30], data);

	    const slice = view.slice(data, data + 3);
	    assert.deepEqual([...slice], [10, 20, 30]);

	    // 3) Free WASM buffer
	    instance.exports.wasm_free(buf);
	});
    });
});
