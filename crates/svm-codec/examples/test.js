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

function loadWasmBufferDataAsJson(instance, buf) {
    let length = wasmBufferLength(instance, buf);
    const slice = wasmBufferDataSlice(instance, buf, 0, length);
    assert.equal(slice[0], OK_MARKER);

    const string = new TextDecoder('utf-8').decode(slice.slice(1));
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

describe('Encode Function Buffer', function () {
    function binToString(array) {
	let result = "";

	for (const b of array) {
	    let s = b.toString(16);

	    // padding
	    if (s.length < 2) {
		s = '0' + s;
	    }

	    result += s
	}
	return result;
    }

    function encodeFuncBuf(instance, object) {
	const buf = wasmNewBuffer(instance, object);
	const result = instanceCall(instance, 'wasm_encode_func_buf', buf);

	const len = wasmBufferLength(instance, result);
	const slice = wasmBufferDataSlice(instance, result, 0, len);
	assert.equal(slice[0], OK_MARKER);

	const data = slice.slice(1)

	wasmBufferFree(instance, buf);
	wasmBufferFree(instance, result);

	return data
    }

    function decodeFuncBuf(instance, encodedData) {
	const object = {
	    data: binToString(encodedData)
	};

	const buf = wasmNewBuffer(instance, object);
	const result = instanceCall(instance, 'wasm_decode_func_buf', buf);
	const json = loadWasmBufferDataAsJson(instance, result);

	wasmBufferFree(instance, buf);
	wasmBufferFree(instance, result);

	return json;
    }
	

    it('address', function () {
	return compileWasmCodec().then(instance => {
	    const object = {
	    	abi: ['address'],
	    	data: ['11233344556677889900AABBCCDDEEFFABCDEFFF'],
	    };	

	    let encoded = encodeFuncBuf(instance, object);
	    let decoded = decodeFuncBuf(instance, encoded);

	    assert.deepEqual(decoded,
	    		 {
	    		     result: [{ address: '11233344556677889900aabbccddeeffabcdefff' }]
	    		 });
	})
    })
})

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
    })
});

describe('Deploy Template', function () {
    it('Encodes & Decodes valid transactions', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {
	      "version": 0,
	      "name": "My Template",
	      "code": "C0DE",
	      "data": "0000000100000003"
	    };

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_deploy_template', buf);

	    let len = wasmBufferLength(instance, result);
	    const slice = wasmBufferDataSlice(instance, result, 0, len);
	    assert.equal(slice[0], OK_MARKER);

	    // `bytes` is a `Uint8Array` holding the encoded `SVM spawn-app` transaction
	    const bytes = slice.slice(1);

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	});
    })
    it('Handles errors for invalid transactions', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {
              version: 0,
	    };

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_deploy_template', buf);

	    let len = wasmBufferLength(instance, result);
	    const slice = wasmBufferDataSlice(instance, result, 0, len);
	    assert.equal(slice[0], ERR_MARKER);

	    const error = new TextDecoder('utf-8').decode(slice.slice(1));
	    assert.equal(error, "InvalidField { field: \"name\", reason: \"value `null` isn\\'t a string\" }");

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	})
    })
});

describe('Spawn App', function () {
    it('Encodes & Decodes valid transactions', function () {
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
    it('Handles errors for invalid transactions', function () {
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
    })
});

describe('Execute App (a.k.a `Call Method`)', function () {
    it('Encodes & Decodes valid transaction', function () {
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
    it('Handles errors for invalid transactions', function () {
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
})

