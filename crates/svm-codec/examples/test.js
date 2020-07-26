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

function loadWasmBufferDataAsString(instance, buf) {
    let length = wasmBufferLength(instance, buf);
    const slice = wasmBufferDataSlice(instance, buf, 0, length);
    assert.equal(slice[0], OK_MARKER);

    const string = new TextDecoder('utf-8').decode(slice.slice(1));
    return string
}

function loadWasmBufferError(instance, buf) {
    let length = wasmBufferLength(instance, buf);
    const slice = wasmBufferDataSlice(instance, buf, 0, length);
    assert.equal(slice[0], ERR_MARKER);

    const string = new TextDecoder('utf-8').decode(slice.slice(1));
    return string
}

function loadWasmBufferDataAsJson(instance, buf) {
    let length = wasmBufferLength(instance, buf);
    const slice = wasmBufferDataSlice(instance, buf, 0, length);

    if (slice[0] == ERR_MARKER) {
	const msg = loadWasmBufferError(instance, buf);
	console.log(msg);

	throw msg;
    }

    assert.equal(slice[0], OK_MARKER);

    const string = loadWasmBufferDataAsString(instance, buf);

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

function repeatString(s, byteLength) {
    const n = s.length;
    const t = byteLength * 2;

    assert(t % n == 0);

    let m = t / n;

    return s.repeat(m)
}

function generateAddress(s) {
    // an `Address` takes 20 bytes
    // which are 40 hexadecimal digits
    return repeatString(s, 20)
}

function generatePubKey256(s) {
    // an `Address` takes 32 bytes
    // which are 64 hexadecimal digits
    return repeatString(s, 32)
}

function encodeCallData(instance, object) {
    const buf = wasmNewBuffer(instance, object);
    const result = instanceCall(instance, 'wasm_encode_calldata', buf);

    const encoded = loadWasmBufferDataAsJson(instance, result);

    wasmBufferFree(instance, buf);
    wasmBufferFree(instance, result);

    return encoded
}

function decodeCallData(instance, encodedData) {
    const buf = wasmNewBuffer(instance, encodedData);
    const result = instanceCall(instance, 'wasm_decode_calldata', buf);
    const json = loadWasmBufferDataAsJson(instance, result);

    wasmBufferFree(instance, buf);
    wasmBufferFree(instance, result);

    return json;
}

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

describe('Encode Function Buffer', function () {
    it('address', function () {
	return compileWasmCodec().then(instance => {
	    const object = {
	    	abi: ['address'],
	    	data: [generateAddress('1020304050')],
	    };	

	    let encoded = encodeCallData(instance, object);
	    let decoded = decodeCallData(instance, encoded);

	    assert.deepEqual(decoded,
	    		 {
	    		     func_args: [],
	    		     func_buf: [{ address: generateAddress('1020304050') }]
	    		 });
	})
    })

    it('pubkey256', function () {
    	return compileWasmCodec().then(instance => {
    	    const object = {
    	    	abi: ['pubkey256'],
    	    	data: [generatePubKey256('10203040')]
    	    };	

    	    let encoded = encodeCallData(instance, object);
    	    let decoded = decodeCallData(instance, encoded);

    	    assert.deepEqual(decoded,
    	    		 {
			     func_args: [],
    	    		     func_buf: [{ pubkey256: generatePubKey256('10203040') }]
    	    		 });
    	})
    })

    it('[address]', function () {
    	return compileWasmCodec().then(instance => {
	    const addr1 = generateAddress('1020304050');
	    const addr2 = generateAddress('a0b0c0d0');

	    const object = {
	    	abi: [ ['address'] ],
	    	data: [ [addr1, addr2] ],
	    };	

	    let encoded = encodeCallData(instance, object);
    	    let decoded = decodeCallData(instance, encoded);

    	    assert.deepEqual(decoded,
    	    		 {
			     func_args: [],
    	    		     func_buf: [ [{address: addr1}, {address: addr2}] ]
    	    		 });
	})
    });

    it('[pubkey256]', function () {
    	return compileWasmCodec().then(instance => {
	    const pkey1 = generatePubKey256('10203040');
	    const pkey2 = generatePubKey256('a0b0c0d0');

	    const object = {
	    	abi: [ ['pubkey256'] ],
	    	data: [ [pkey1, pkey2] ],
	    };	

	    let encoded = encodeCallData(instance, object);
    	    let decoded = decodeCallData(instance, encoded);

    	    assert.deepEqual(decoded,
    	    		 {
			     func_args: [],
    	    		     func_buf: [ [{pubkey256: pkey1}, {pubkey256: pkey2}] ]
    	    		 });
	})
    });

    it('[address, [address], pubkey256]', function () {
    	return compileWasmCodec().then(instance => {
	    const addr1 = generateAddress('1020304050');
	    const addr2 = generateAddress('a0b0c0d0');
	    const addr3 = generateAddress('aabbccdd');
	    const pkey1 = generatePubKey256('60708090');

	    const object = {
	    	abi: ['address', ['address'], 'pubkey256'],
	    	data: [addr1, [addr2, addr3], pkey1],
	    };	

	    let encoded = encodeCallData(instance, object);
    	    let decoded = decodeCallData(instance, encoded);

    	    assert.deepEqual(decoded,
    	    		 {
			     func_args: [],
    	    		     func_buf: [{address: addr1}, [{address: addr2}, {address: addr3}], {pubkey256: pkey1}] 
    	    		 });
	})
    });
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

	    const error = loadWasmBufferError(instance, result);
	    assert.equal(error, "InvalidField { field: \"name\", reason: \"value `null` isn\\'t a string\" }");

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	})
    })
});

describe('Spawn App', function () {
    function encodeSpawnApp(instance, template, name, calldata) {
	let tx = {
	    version: 0,
	    template: template,
	    name: name,
	    ctor_index: 1,
	    ctor_buf: calldata['func_buf'],
	    ctor_args: calldata['func_args']
	};

	const buf = wasmNewBuffer(instance, tx);
	const result = instanceCall(instance, 'wasm_encode_spawn_app', buf);

	let len = wasmBufferLength(instance, result);
	const slice = wasmBufferDataSlice(instance, result, 0, len);
	assert.equal(slice[0], OK_MARKER);

	wasmBufferFree(instance, buf);
	wasmBufferFree(instance, result);

	return slice.slice(1);
    }

    function decodeSpawnApp(instance, bytes) {
	const data = binToString(bytes);

	const buf = wasmNewBuffer(instance, {data: data});
	const result = instanceCall(instance, 'wasm_decode_spawn_app', buf);
	const json = loadWasmBufferDataAsJson(instance, result);

	wasmBufferFree(instance, buf);
	wasmBufferFree(instance, result);

	return json
    }

    it('Encodes & Decodes valid transactions', function () {
	return compileWasmCodec().then(instance => {
	    const template = generateAddress('1020304050');
	    const pkey = generatePubKey256('11223344');
	    const name = 'My App';

	    const object = {
	    	abi: ['i32', 'pubkey256', 'i64'],
	    	data: [10, pkey, 20],
	    };	

	    let calldata = encodeCallData(instance, object);
	    const bytes = encodeSpawnApp(instance, template, name, calldata);
	    const json = decodeSpawnApp(instance, bytes);

	    assert.deepEqual(json,
			     {
				 version: 0,
				 template: template,
				 name: name,
				 ctor_index: 1,
				 ctor_args: ['10i32', '20i64'],
				 ctor_buf: [{pubkey256: pkey}],
			     });
	});
    })
    it('Handles errors for invalid transactions', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {
              version: 0,
              template: '102030',
	    };

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_encode_spawn_app', buf);

	    const error = loadWasmBufferError(instance, result);
	    assert.equal(error, "InvalidField { field: \"template\", reason: \"value should be exactly 40 hex digits\" }");

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	});
    })
});

describe('Execute App (a.k.a `Call Method`)', function () {
    function encodeExecApp(instance, app, calldata) {
	let tx = {
	    version: 0,
	    app: app,
	    func_index: 1,
	    func_buf: calldata['func_buf'],
	    func_args: calldata['func_args']
	};

	const buf = wasmNewBuffer(instance, tx);
	const result = instanceCall(instance, 'wasm_encode_exec_app', buf);

	let len = wasmBufferLength(instance, result);
	const slice = wasmBufferDataSlice(instance, result, 0, len);
	assert.equal(slice[0], OK_MARKER);

	wasmBufferFree(instance, buf);
	wasmBufferFree(instance, result);

	return slice.slice(1);
    }

    function decodeExecApp(instance, bytes) {
	const data = binToString(bytes);

	const buf = wasmNewBuffer(instance, {data: data});
	const result = instanceCall(instance, 'wasm_decode_exec_app', buf);
	const json = loadWasmBufferDataAsJson(instance, result);

	wasmBufferFree(instance, buf);
	wasmBufferFree(instance, result);

	return json
    }

    it('Encodes & Decodes valid transaction', function () {
	return compileWasmCodec().then(instance => {
	    const app = generateAddress('1020304050');
	    const pkey = generatePubKey256('11223344');

	    const object = {
	    	abi: ['i32', 'pubkey256', 'i64'],
	    	data: [10, pkey, 20],
	    };	

	    let calldata = encodeCallData(instance, object);
	    const bytes = encodeExecApp(instance, app, calldata);
	    const json = decodeExecApp(instance, bytes);

	    assert.deepEqual(json,
			     {
				 version: 0,
				 app: app,
				 func_index: 1,
				 func_args: ['10i32', '20i64'],
				 func_buf: [{pubkey256: pkey}],
			     });
	});
    })
    it('Handles errors for invalid transactions', function () {
	return compileWasmCodec().then(instance => {
	    let tx = {version: 0, app: '102030'};

	    const buf = wasmNewBuffer(instance, tx);
	    const result = instanceCall(instance, 'wasm_encode_exec_app', buf);

	    const error = loadWasmBufferError(instance, result);
	    assert.equal(error, "InvalidField { field: \"app\", reason: \"value should be exactly 40 hex digits\" }");

	    wasmBufferFree(instance, buf);
	    wasmBufferFree(instance, result);
	});
    });
})

