"use strict";

const assert = require("assert");
const fs = require("fs");

const OK_MARKER = 1;
const ERR_MARKER = 0;

// Compiles and instantiates the svm_codec instance for use from javascript
async function compileWasmCodec() {
  const wasm = await WebAssembly.compile(fs.readFileSync("svm_codec.wasm"));
  const importObject = {};

  return WebAssembly.instantiate(wasm, importObject);
}

// Call a function on svm_codec instance with the provided buffer.
// Returns ???
function instanceCall(instance, func_name, buf) {
  const func = instance.exports[func_name];
  return func(buf);
}

// Creates a wasm buffer that can be passed to svm_codec instance methods from a provided json object.
// Returns the buffer.
function wasmNewBuffer(instance, object) {
  const objectStr = JSON.stringify(object);
  const bytes = new TextEncoder("utf-8").encode(objectStr);
  const buf = wasmBufferAlloc(instance, bytes.length);

  assert.strictEqual(bytes.length, wasmBufferLength(instance, buf));

  copyToWasmBufferData(instance, buf, bytes);

  return buf;
}

// Returns a json object from a provided wasm buffer
function loadWasmBuffer(instance, buf) {
  let length = wasmBufferLength(instance, buf);
  const slice = wasmBufferDataSlice(instance, buf, 0, length);
  const string = new TextDecoder("utf-8").decode(slice);

  return JSON.parse(string);
}

// Returns a utf-8 string representation of the data in a svm_codec buffer
function loadWasmBufferDataAsString(instance, buf) {
  let length = wasmBufferLength(instance, buf);
  const slice = wasmBufferDataSlice(instance, buf, 0, length);
  assert.strictEqual(slice[0], OK_MARKER);

  const string = new TextDecoder("utf-8").decode(slice.slice(1));
  return string;
}

// Returns a utf-8 string representation of an error in a svm_codec buffer
function loadWasmBufferError(instance, buf) {
  let length = wasmBufferLength(instance, buf);
  const slice = wasmBufferDataSlice(instance, buf, 0, length);
  assert.strictEqual(slice[0], ERR_MARKER);

  const string = new TextDecoder("utf-8").decode(slice.slice(1));
  return string;
}

// Returns a json object representation of the data in a svm_codec buffer
// Throws an exception if buffer has an error section with the exception string representation
function loadWasmBufferDataAsJson(instance, buf) {
  let length = wasmBufferLength(instance, buf);
  const slice = wasmBufferDataSlice(instance, buf, 0, length);

  if (slice[0] == ERR_MARKER) {
    const msg = loadWasmBufferError(instance, buf);
    console.log(msg);

    throw msg;
  }

  assert.strictEqual(slice[0], OK_MARKER);

  const string = loadWasmBufferDataAsString(instance, buf);

  return JSON.parse(string);
}

// Allocates a svm_codec buffer with the provided byte length
function wasmBufferAlloc(instance, length) {
  return instance.exports.wasm_alloc(length);
}

// Frees an allocated svm_codec buffer that was previosuly allocated
function wasmBufferFree(instance, buf) {
  return instance.exports.wasm_free(buf);
}

// Returns the length in bytes of a wasm_codec buffer
function wasmBufferLength(instance, buf) {
  return instance.exports.wasm_buffer_length(buf);
}

// Frees the data allocated in a svm_codec buffer
function wasmBufferDataPtr(instance, buf) {
  return instance.exports.wasm_buffer_data(buf);
}

// ????
function copyToWasmBufferData(instance, buf, data) {
  let ptr = wasmBufferDataPtr(instance, buf);
  let memory = instance.exports.memory.buffer;
  let view = new Uint8Array(memory);
  view.set([...data], ptr);
}

// ????
function wasmBufferDataSlice(instance, buf, offset, length) {
  let ptr = wasmBufferDataPtr(instance, buf);

  const memory = instance.exports.memory.buffer;
  const view = new Uint8Array(memory);
  const slice = view.slice(ptr + offset, ptr + offset + length);

  return slice;
}

// ????
function repeatString(s, byteLength) {
  const n = s.length;
  const t = byteLength * 2;

  assert(t % n == 0);

  let m = t / n;

  return s.repeat(m);
}

// Returns a 20 bytes address (40 hex digits)
// Note: in Spacemesh, address should be the last 20 bytes of a public key, not the first 20
function generateAddress(s) {
  // an `Address` takes 20 bytes
  // which are 40 hexadecimal digits
  return repeatString(s, 20);
}

// ????
function encodeInput(instance, object) {
  const buf = wasmNewBuffer(instance, object);
  const result = instanceCall(instance, "wasm_encode_inputdata", buf);

  const encoded = loadWasmBufferDataAsJson(instance, result);

  wasmBufferFree(instance, buf);
  wasmBufferFree(instance, result);

  return encoded;
}

// ????
function decodeInput(instance, encodedData) {
  const buf = wasmNewBuffer(instance, encodedData);
  const result = instanceCall(instance, "wasm_decode_inputdata", buf);
  const json = loadWasmBufferDataAsJson(instance, result);

  wasmBufferFree(instance, buf);
  wasmBufferFree(instance, result);

  return json;
}

// ????
function binToString(array) {
  let result = "";

  for (const b of array) {

    // toString takes no arg????
    let s = b.toString(16);

    // padding
    if (s.length < 2) {
      s = "0" + s;
    }

    result += s;
  }
  return result;
}

////// Tests

describe("Encode InputData", function () {
  function testInputData(instance, abi, data) {
    const calldata = {
      abi: abi,
      data: data,
    };

    let encoded = encodeInput(instance, calldata);
    let decoded = decodeInput(instance, encoded);

    assert.deepStrictEqual(decoded, calldata);
  }

  it("i8", function () {
    return compileWasmCodec().then((instance) => {
      testInputData(instance, ["i8"], [-10]);
    });
  });

  it("u8", function () {
    return compileWasmCodec().then((instance) => {
      testInputData(instance, ["u8"], [10]);
    });
  });

  it("i16", function () {
    return compileWasmCodec().then((instance) => {
      testInputData(instance, ["i16"], [-10]);
    });
  });

  it("u16", function () {
    return compileWasmCodec().then((instance) => {
      testInputData(instance, ["u16"], [10]);
    });
  });

  it("i32", function () {
    return compileWasmCodec().then((instance) => {
      testInputData(instance, ["i32"], [-10]);
    });
  });

  it("u32", function () {
    return compileWasmCodec().then((instance) => {
      testInputData(instance, ["u32"], [10]);
    });
  });

  it("amount", function () {
    return compileWasmCodec().then((instance) => {
      testInputData(instance, ["amount"], [10]);
    });
  });

  it("address", function () {
    return compileWasmCodec().then((instance) => {
      const addr = generateAddress("1020304050");
      const object = {
        abi: ["address"],
        data: [addr],
      };

      let encoded = encodeInput(instance, object);
      let decoded = decodeInput(instance, encoded);

      assert.deepStrictEqual(decoded, {
        abi: ["address"],
        data: [addr],
      });
    });
  });

  it("[address]", function () {
    return compileWasmCodec().then((instance) => {
      const addr1 = generateAddress("1020304050");
      const addr2 = generateAddress("A0B0C0D0");

      const object = {
        abi: [["address"]],
        data: [[addr1, addr2]],
      };

      let encoded = encodeInput(instance, object);
      let decoded = decodeInput(instance, encoded);

      assert.deepStrictEqual(decoded, {
        abi: [["address"]],
        data: [[addr1, addr2]],
      });
    });
  });
});

describe("WASM Buffer", function () {
  it("Allocate & Free", function () {
    return compileWasmCodec().then((instance) => {
      let object = {
        message: "Hello World",
        status: 200,
      };

      const buf = wasmNewBuffer(instance, object);
      const loaded = loadWasmBuffer(instance, buf);
      assert.deepStrictEqual(loaded, object);

      wasmBufferFree(instance, buf);
    });
  });
});

describe("Deploy Template", function () {
  it("Encodes & Decodes valid transactions", function () {
    return compileWasmCodec().then((instance) => {
      let tx = {
        svm_version: 1,
        code_version: 2,
        name: "My Template",
        desc: "A few words",
        code: "C0DE",
        data: "0000000100000003",
        ctors: ["init", "start"],
      };

      const buf = wasmNewBuffer(instance, tx);
      const result = instanceCall(instance, "wasm_encode_deploy", buf);

      let len = wasmBufferLength(instance, result);
      const slice = wasmBufferDataSlice(instance, result, 0, len);
      assert.strictEqual(slice[0], OK_MARKER);

      // `bytes` is a `Uint8Array` holding the encoded `SVM spawn-account` transaction
      const bytes = slice.slice(1);

      wasmBufferFree(instance, buf);
      wasmBufferFree(instance, result);
    });
  });
  it("Handles errors for invalid transactions", function () {
    return compileWasmCodec().then((instance) => {
      let tx = {
        svm_version: 1,
        code_version: 2,
      };

      const buf = wasmNewBuffer(instance, tx);
      const result = instanceCall(instance, "wasm_encode_deploy", buf);

      const error = loadWasmBufferError(instance, result);
      assert.strictEqual(error, "A non-optional field was missing (`name`).");

      wasmBufferFree(instance, buf);
      wasmBufferFree(instance, result);
    });
  });
});

describe("Spawn Account", function () {
  function encodeSpawn(instance, template, name, calldata) {
    let tx = {
      version: 0,
      template: template,
      name: name,
      ctor_name: "initialize",
      calldata: calldata,
    };

    const buf = wasmNewBuffer(instance, tx);
    const result = instanceCall(instance, "wasm_encode_spawn", buf);

    let len = wasmBufferLength(instance, result);
    const slice = wasmBufferDataSlice(instance, result, 0, len);
    assert.strictEqual(slice[0], OK_MARKER);

    wasmBufferFree(instance, buf);
    wasmBufferFree(instance, result);

    return slice.slice(1);
  }

  function decodeSpawn(instance, bytes) {
    const data = binToString(bytes);

    const buf = wasmNewBuffer(instance, { data: data });
    const result = instanceCall(instance, "wasm_decode_spawn", buf);
    const json = loadWasmBufferDataAsJson(instance, result);

    wasmBufferFree(instance, buf);
    wasmBufferFree(instance, result);

    return json;
  }

  it("Encodes & Decodes valid transactions", function () {
    return compileWasmCodec().then((instance) => {
      const template = generateAddress("1020304050");
      const name = "My Account";

      const object = {
        abi: ["i32", "i64"],
        data: [10, 20],
      };

      let calldata = encodeInput(instance, object);
      const bytes = encodeSpawn(instance, template, name, calldata["data"]);
      const json = decodeSpawn(instance, bytes);

      assert.deepStrictEqual(json, {
        version: 0,
        template: template,
        name: name,
        ctor_name: "initialize",
        calldata: {
          abi: ["i32", "i64"],
          data: [10, 20],
        },
      });
    });
  });
  it("Handles errors for invalid transactions", function () {
    return compileWasmCodec().then((instance) => {
      let tx = {
        version: 0,
        template: "102030",
      };

      const buf = wasmNewBuffer(instance, tx);
      const result = instanceCall(instance, "wasm_encode_spawn", buf);

      const error = loadWasmBufferError(instance, result);
      assert.strictEqual(
        error,
        "The value of a specific field is invalid (`template`)."
      );

      wasmBufferFree(instance, buf);
      wasmBufferFree(instance, result);
    });
  });
});

describe("Call Account", function () {
  function encodeCall(instance, target, verifydata, calldata) {
    let tx = {
      version: 0,
      target: target,
      func_name: "do_something",
      verifydata: verifydata,
      calldata: calldata,
    };

    const buf = wasmNewBuffer(instance, tx);
    const result = instanceCall(instance, "wasm_encode_call", buf);

    let len = wasmBufferLength(instance, result);
    const slice = wasmBufferDataSlice(instance, result, 0, len);
    assert.strictEqual(slice[0], OK_MARKER);

    wasmBufferFree(instance, buf);
    wasmBufferFree(instance, result);

    return slice.slice(1);
  }

  function decodeCall(instance, bytes) {
    const data = binToString(bytes);

    const buf = wasmNewBuffer(instance, { data: data });
    const result = instanceCall(instance, "wasm_decode_call", buf);
    const json = loadWasmBufferDataAsJson(instance, result);

    wasmBufferFree(instance, buf);
    wasmBufferFree(instance, result);

    return json;
  }

  it("Encodes & Decodes valid transaction", function () {
    return compileWasmCodec().then((instance) => {
      const target = generateAddress("1020304050");

      let verifydata = encodeInput(instance, {
        abi: ["bool", "i8"],
        data: [true, 5],
      });

      let calldata = encodeInput(instance, {
        abi: ["i32", "i64"],
        data: [10, 20],
      });

      const bytes = encodeCall(
        instance,
        target,
        verifydata["data"],
        calldata["data"]
      );
      const json = decodeCall(instance, bytes);

      assert.deepStrictEqual(json, {
        version: 0,
        target: target,
        func_name: "do_something",
        verifydata: {
          abi: ["bool", "i8"],
          data: [true, 5],
        },
        calldata: {
          abi: ["i32", "i64"],
          data: [10, 20],
        },
      });
    });
  });
  it("Handles errors for invalid transactions", function () {
    return compileWasmCodec().then((instance) => {
      let tx = { version: 0, target: "102030" };

      const buf = wasmNewBuffer(instance, tx);
      const result = instanceCall(instance, "wasm_encode_call", buf);

      const error = loadWasmBufferError(instance, result);
      assert.strictEqual(
        error,
        "The value of a specific field is invalid (`target`)."
      );

      wasmBufferFree(instance, buf);
      wasmBufferFree(instance, result);
    });
  });
});
