"use strict";

const assert = require("assert");
const fs = require("fs");

const OK_MARKER = 1;
const ERR_MARKER = 0;

async function compileWasmCodec() {
  const wasm = await WebAssembly.compile(fs.readFileSync("svm_codec.wasm"));
  const importObject = {};

  return WebAssembly.instantiate(wasm, importObject);
}

function instanceCall(instance, func_name, buf) {
  const func = instance.exports[func_name];
  return func(buf);
}

function wasmNewBuffer(instance, object) {
  const objectStr = JSON.stringify(object);
  const bytes = new TextEncoder("utf-8").encode(objectStr);
  const buf = wasmBufferAlloc(instance, bytes.length);

  assert.strictEqual(bytes.length, wasmBufferLength(instance, buf));

  copyToWasmBufferData(instance, buf, bytes);

  return buf;
}

function loadWasmBuffer(instance, buf) {
  let length = wasmBufferLength(instance, buf);
  const slice = wasmBufferDataSlice(instance, buf, 0, length);
  const string = new TextDecoder("utf-8").decode(slice);

  return JSON.parse(string);
}

function loadWasmBufferDataAsString(instance, buf) {
  let length = wasmBufferLength(instance, buf);
  const slice = wasmBufferDataSlice(instance, buf, 0, length);
  assert.strictEqual(slice[0], OK_MARKER);

  const string = new TextDecoder("utf-8").decode(slice.slice(1));
  return string;
}

function loadWasmBufferError(instance, buf) {
  let length = wasmBufferLength(instance, buf);
  const slice = wasmBufferDataSlice(instance, buf, 0, length);
  assert.strictEqual(slice[0], ERR_MARKER);

  const string = new TextDecoder("utf-8").decode(slice.slice(1));
  return string;
}

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

  return slice;
}

function repeatString(s, byteLength) {
  const n = s.length;
  const t = byteLength * 2;

  assert(t % n == 0);

  let m = t / n;

  return s.repeat(m);
}

function generateAddress(s) {
  // an `Address` takes 20 bytes
  // which are 40 hexadecimal digits
  return repeatString(s, 20);
}

function encodeCallData(instance, object) {
  const buf = wasmNewBuffer(instance, object);
  const result = instanceCall(instance, "wasm_encode_calldata", buf);

  const encoded = loadWasmBufferDataAsJson(instance, result);

  wasmBufferFree(instance, buf);
  wasmBufferFree(instance, result);

  return encoded;
}

function decodeCallData(instance, encodedData) {
  const buf = wasmNewBuffer(instance, encodedData);
  const result = instanceCall(instance, "wasm_decode_calldata", buf);
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
      s = "0" + s;
    }

    result += s;
  }
  return result;
}

describe("Encode Calldata", function () {
  function testCallData(instance, abi, data) {
    const calldata = {
      abi: abi,
      data: data,
    };

    let encoded = encodeCallData(instance, calldata);
    let decoded = decodeCallData(instance, encoded);

    assert.deepStrictEqual(decoded, calldata);
  }

  it("i8", function () {
    return compileWasmCodec().then((instance) => {
      testCallData(instance, ["i8"], [-10]);
    });
  });

  it("u8", function () {
    return compileWasmCodec().then((instance) => {
      testCallData(instance, ["u8"], [10]);
    });
  });

  it("i16", function () {
    return compileWasmCodec().then((instance) => {
      testCallData(instance, ["i16"], [-10]);
    });
  });

  it("u16", function () {
    return compileWasmCodec().then((instance) => {
      testCallData(instance, ["u16"], [10]);
    });
  });

  it("i32", function () {
    return compileWasmCodec().then((instance) => {
      testCallData(instance, ["i32"], [-10]);
    });
  });

  it("u32", function () {
    return compileWasmCodec().then((instance) => {
      testCallData(instance, ["u32"], [10]);
    });
  });

  it("amount", function () {
    return compileWasmCodec().then((instance) => {
      testCallData(instance, ["amount"], [10]);
    });
  });

  it("address", function () {
    return compileWasmCodec().then((instance) => {
      const addr = generateAddress("1020304050");
      const object = {
        abi: ["address"],
        data: [addr],
      };

      let encoded = encodeCallData(instance, object);
      let decoded = decodeCallData(instance, encoded);

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

      let encoded = encodeCallData(instance, object);
      let decoded = decodeCallData(instance, encoded);

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
      assert.strictEqual(
        error,
        'InvalidField { field: "name", reason: "value `null` isn\'t a string" }'
      );

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

      let calldata = encodeCallData(instance, object);
      const bytes = encodeSpawn(instance, template, name, calldata["calldata"]);
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
        'InvalidField { field: "template", reason: "value should be exactly 40 hex digits" }'
      );

      wasmBufferFree(instance, buf);
      wasmBufferFree(instance, result);
    });
  });
});

describe("Call Account", function () {
  function encodeCall(instance, account, calldata) {
    let tx = {
      version: 0,
      account: account,
      func_name: "do_something",
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

      const object = {
        abi: ["i32", "i64"],
        data: [10, 20],
      };

      let calldata = encodeCallData(instance, object);
      const bytes = encodeCall(instance, target, calldata["calldata"]);
      const json = decodeCall(instance, bytes);

      assert.deepStrictEqual(json, {
        version: 0,
        target: target,
        func_name: "do_something",
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
        'InvalidField { field: "target", reason: "value should be exactly 40 hex digits" }'
      );

      wasmBufferFree(instance, buf);
      wasmBufferFree(instance, result);
    });
  });
});
