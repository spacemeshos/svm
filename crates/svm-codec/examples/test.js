'use strict';

const assert = require('assert');
const fs = require('fs');

const OK_MARKER = 1;
const ERR_MARKER = 0;

async function compileWasmCodec () {
    const wasm = await WebAssembly.compile(fs.readFileSync('svm_codec.wasm'));
    const importObject = {};

    return WebAssembly.instantiate(wasm, importObject)
}

function instanceCall(instance, func_name, buf) {
    const func = instance.exports[func_name];
    return func(buf)
}

function wasmNewBuffer(instance, object) {
    const objectStr = JSON.stringify(object);
    const bytes = new TextEncoder('utf-8').encode(objectStr);
    const buf = wasmBufferAlloc(instance, bytes.length);

    assert.equal(bytes.length, wasmBufferLength(instance, buf));

    copyToWasmBufferData(instance, buf, bytes); 

    return buf;
}   

function loadWasmBuffer(instance, buf) {
    let length = wasmBufferLength(instance, buf);
    const slice = wasmBufferDataSlice(instance, buf, 0, length);
    const string = new TextDecoder('utf-8').decode(slice);

    return JSON.parse(string)
}

function wasmBufferAlloc(instance, length) {
    return instance.exports.wasm_alloc(length);
}

function wasmBufferFree(instance, buf) {
    return instance.exports.wasm_free(buf);
}

function wasmBufferLength(instance, buf) {
    return instance.exports.wasm_buffer_length(buf);
}

function wasmBufferDataPtr(instance, buf) {
    return instance.exports.wasm_buffer_data(buf);
}

function copyToWasmBufferData(instance, buf, data) {
    let ptr = wasmBufferDataPtr(instance, buf);
    let memory = instance.exports.memory.buffer;
    let view = new Uint8Array(memory);
    view.set([...data], ptr);
}

function wasmBufferDataSlice(instance, buf, offset, length) {
    let ptr = wasmBufferDataPtr(instance, buf);

    const memory = instance.exports.memory.buffer;
    const view = new Uint8Array(memory);
    const slice = view.slice(ptr + offset, ptr + offset + length);

    return slice
}

describe('WASM Buffer', function () {
    it('Allocate & Free', function () {
	return compileWasmCodec().then(instance => {
	    let object = {
		message: 'Hello World',
		status: 200,
	    };

	    const buf = wasmNewBuffer(instance, object);
	    const loaded = loadWasmBuffer(instance, buf);
	    assert.deepEqual(loaded, object);

	    wasmBufferFree(instance, buf);
	})
    }),
    it('Encodes `spawn-app` valid transaction', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {
              version: 0,
              template: "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
              ctor_index: 1,
              ctor_buf: "A2B3",
              ctor_args: ["10i32", "20i64"]
	    };

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_spawn_app', buf);

	    let len = wasmBufferLength(instance, result);
	    const slice = wasmBufferDataSlice(instance, result, 0, len);
	    assert.equal(slice[0], OK_MARKER);

	    // `bytes` is a `Uint8Array` holding the encoded `SVM spawn-app` transaction
	    const bytes = slice.slice(1);

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	});
    })
    it('Encodes `exec-app` valid transaction', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {
              version: 0,
              app: "10203040506070809000A0B0C0D0E0F0ABCDEFFF",
              func_index: 1,
              func_buf: "A2B3",
              func_args: ["10i32", "20i64"]
	    };

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_exec_app', buf);

	    let len = wasmBufferLength(instance, result);
	    const slice = wasmBufferDataSlice(instance, result, 0, len);
	    assert.equal(slice[0], OK_MARKER);

	    // `bytes` is a `Uint8Array` holding the encoded `SVM exec-app` transaction
	    const bytes = slice.slice(1);

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	});
    })
    it('Encodes `spawn-app` invalid transaction', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {
              version: 0,
              template: "102030",
	    };

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_spawn_app', buf);

	    let len = wasmBufferLength(instance, result);
	    const slice = wasmBufferDataSlice(instance, result, 0, len);
	    assert.equal(slice[0], ERR_MARKER);

	    const error = new TextDecoder('utf-8').decode(slice.slice(1));
	    assert.equal(error, "InvalidField { field: \"template\", reason: \"value should be exactly 40 hex digits\" }");

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	});
    });
    it('Encodes `exec-app` invalid transaction', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {
              version: 0,
              app: "102030",
	    };

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_exec_app', buf);

	    let len = wasmBufferLength(instance, result);
	    const slice = wasmBufferDataSlice(instance, result, 0, len);
	    assert.equal(slice[0], ERR_MARKER);

	    const error = new TextDecoder('utf-8').decode(slice.slice(1));
	    assert.equal(error, "InvalidField { field: \"app\", reason: \"value should be exactly 40 hex digits\" }");

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	});
    });
});
