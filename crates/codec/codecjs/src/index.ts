"use strict";

import { compileFunction } from "vm";

const assert = require("assert");
const fs = require("fs");

//function loadWasmBuffer(instance, buf) {
//  let length = wasmBufferLength(instance, buf);
//  const slice = wasmBufferDataSlice(instance, buf, 0, length);
//  const string = new TextDecoder().decode(slice);
//
//  return JSON.parse(string);
//}
//
//function loadWasmBufferDataAsString(instance, buf) {
//  let length = wasmBufferLength(instance, buf);
//  const slice = wasmBufferDataSlice(instance, buf, 0, length);
//  assert.strictEqual(slice[0], OK_MARKER);
//
//  const string = new TextDecoder().decode(slice.slice(1));
//  return string;
//}
//
//function loadWasmBufferError(instance, buf) {
//  let length = wasmBufferLength(instance, buf);
//  const slice = wasmBufferDataSlice(instance, buf, 0, length);
//  assert.strictEqual(slice[0], ERR_MARKER);
//
//  const string = new TextDecoder().decode(slice.slice(1));
//  return string;
//}
//
//function loadWasmBufferDataAsJson(instance, buf) {
//  let length = wasmBufferLength(instance, buf);
//  const slice = wasmBufferDataSlice(instance, buf, 0, length);
//
//  if (slice[0] == ERR_MARKER) {
//    const msg = loadWasmBufferError(instance, buf);
//    console.log(msg);
//
//    throw msg;
//  }
//
//  assert.strictEqual(slice[0], OK_MARKER);
//
//  const string = loadWasmBufferDataAsString(instance, buf);
//
//  return JSON.parse(string);
//}
//

interface SvmWasmApi {
  wasm_encode_input(offset: number): number;
  wasm_decode_input(offset: number): number;
  wasm_encode_spawn(offset: number): number;
  wasm_decode_spawn(offset: number): number;
  wasm_encode_call(offset: number): number;
  wasm_decode_call(offset: number): number;
  wasm_decode_receipt(offset: number): number;
}

/**
 * 
 */
class Svm {
  wasm!: WebAssembly.WebAssemblyInstantiatedSource;
  api!: SvmWasmApi;
  encoder: TextEncoder = new TextEncoder();

  /**
   * 
   * @returns 
   */
  static compile = async () => {
    const svm = new Svm();
    let code = fs.readFileSync("svm_codec.wasm");
    let importOject = {};
    svm.wasm = await WebAssembly.instantiate(code, importOject);
    return svm;
  };

  private exports(): WebAssembly.Exports {
    return this.wasm.instance.exports;
  }

  /**
   * 
   * @param data 
   */
  encodeInput(data: Uint8Array) {
    let dataAsJson = {
      "data": data
    };
    let encode = this.exports().encode_input;
    let json = this.encoder.encode(JSON.stringify(dataAsJson));
    let buf = this.alloc(json)
    this.exports().encode_input(buf.offset);
  }

  /**
   * 
   * @param encodedData 
   */
  decodeInput(encodedData: Uint8Array) {
    let decode = this.exports().wasm_decode_input;
    let buf = this.alloc(encodedData);
    decode(buf.offset);
  }

  /**
   * 
   * @param encodedReceipt 
   */
  decodeReceipt(encodedReceipt: Uint8Array) {
    let decode: ((bufferOffset: number) => number) = this.exports.wasm_decode_receipt;
    let buf = this.alloc(encodedReceipt);
    decode(buf.offset);
  }

  /**
   * 
   * @param encodedSpawn 
   */
  decodeSpawn(encodedSpawn: Uint8Array) {
    let decode: ((bufferOffset: number) => number) = this.exports.wasm_decode_input;
    let buf = this.alloc(encodedSpawn);
    decode(buf.offset);
  }

  /**
   * 
   * @param spawn 
   */
  encodeSpawn(spawn: object) {
    let encode: ((bufferOffset: number) => number) = this.exports.wasm_encode_spawn;
    let json = this.encoder.encode(JSON.stringify(spawn));
    let buf = this.alloc(json)
    encode(buf.offset);
  }

  /**
   * 
   * @param deploy 
   */
  encodeDeploy(deploy: object) {
    let encode: ((bufferOffset: number) => number) = this.exports.wasm_encode_deploy;
    let json = this.encoder.encode(JSON.stringify(deploy));
    let buf = this.alloc(json)
    encode(buf.offset);
  }

  /**
   * 
   * @param encodedCall 
   */
  decodeCall(encodedCall: Uint8Array) {
    let decode: ((bufferOffset: number) => number) = this.exports.wasm_decode_call;
    let buf = this.alloc(encodedCall);
    decode(buf.offset);
  }

  /**
   * 
   * @param call 
   */
  encodeCall(call: object) {
    let encode: ((bufferOffset: number) => number) = this.exports.wasm_encode_call;
    let json = this.encoder.encode(JSON.stringify(call));
    let buf = this.alloc(json)
    encode(buf.offset);
  }

  private alloc(contents: Uint8Array): SvmBuffer {
    let alloc: (() => number) = this.exports.wasm_alloc;
    let offset = alloc();
    let buf = new SvmBuffer(offset, this);
    return new SvmBuffer(alloc(), this);
  }

  private free(buf: SvmBuffer) {
    let free: ((offset: number) => void) = this.exports.wasm_free;
    free(buf.offset);
  }
}

class SvmBuffer {
  offset: number = 0;
  svm: Svm;

  private OK_MARKER: number = 1;
  private ERR_MARKER: number = 1;

  constructor(offset: number, svm: Svm) {
    this.offset = offset;
    this.svm = svm;
  }

  clone(): Uint8Array {
    let memory = this.svm.exports.memory.buffer;
    return new Uint8Array(memory);
  }
}

//
//function copyToWasmBufferData(instance, buf, data) {
//  let ptr = wasmBufferDataPtr(instance, buf);
//  let memory = instance.exports.memory.buffer;
//  let view = new Uint8Array(memory);
//  view.set([...data], ptr);
//}
//
//function wasmBufferDataSlice(instance, buf, offset, length) {
//  let ptr = wasmBufferDataPtr(instance, buf);
//
//  const memory = instance.exports.memory.buffer;
//  const view = new Uint8Array(memory);
//  const slice = view.slice(ptr + offset, ptr + offset + length);
//
//  return slice;
//}

//function binToString(array) {
//  let result = "";
//
//  for (const b of array) {
//    let s = b.toString(16);
//
//    // padding
//    if (s.length < 2) {
//      s = "0" + s;
//    }
//
//    result += s;
//  }
//  return result;
//}
//
//describe("Encode InputData", function () {
//  function testInputData(instance, abi, data) {
//    const calldata = {
//      abi: abi,
//      data: data,
//    };
//
//    let encoded = encodeInput(instance, calldata);
//    let decoded = decodeInput(instance, encoded);
//
//    assert.deepStrictEqual(decoded, calldata);
//  }
//
//  it("i8", function () {
//    return compileWasmCodec().then((instance) => {
//      testInputData(instance, ["i8"], [-10]);
//    });
//  });
//
//  it("u8", function () {
//    return compileWasmCodec().then((instance) => {
//      testInputData(instance, ["u8"], [10]);
//    });
//  });
//
//  it("i16", function () {
//    return compileWasmCodec().then((instance) => {
//      testInputData(instance, ["i16"], [-10]);
//    });
//  });
//
//  it("u16", function () {
//    return compileWasmCodec().then((instance) => {
//      testInputData(instance, ["u16"], [10]);
//    });
//  });
//
//  it("i32", function () {
//    return compileWasmCodec().then((instance) => {
//      testInputData(instance, ["i32"], [-10]);
//    });
//  });
//
//  it("u32", function () {
//    return compileWasmCodec().then((instance) => {
//      testInputData(instance, ["u32"], [10]);
//    });
//  });
//
//  it("amount", function () {
//    return compileWasmCodec().then((instance) => {
//      testInputData(instance, ["amount"], [10]);
//    });
//  });
//
//  it("address", function () {
//    return compileWasmCodec().then((instance) => {
//      const addr = generateAddress("1020304050");
//      const object = {
//        abi: ["address"],
//        data: [addr],
//      };
//
//      let encoded = encodeInput(instance, object);
//      let decoded = decodeInput(instance, encoded);
//
//      assert.deepStrictEqual(decoded, {
//        abi: ["address"],
//        data: [addr],
//      });
//    });
//  });
//
//  it("[address]", function () {
//    return compileWasmCodec().then((instance) => {
//      const addr1 = generateAddress("1020304050");
//      const addr2 = generateAddress("A0B0C0D0");
//
//      const object = {
//        abi: [["address"]],
//        data: [[addr1, addr2]],
//      };
//
//      let encoded = encodeInput(instance, object);
//      let decoded = decodeInput(instance, encoded);
//
//      assert.deepStrictEqual(decoded, {
//        abi: [["address"]],
//        data: [[addr1, addr2]],
//      });
//    });
//  });
//});
//
//describe("WASM Buffer", function () {
//  it("Allocate & Free", function () {
//    return compileWasmCodec().then((instance) => {
//      let object = {
//        message: "Hello World",
//        status: 200,
//      };
//
//      const buf = wasmNewBuffer(instance, object);
//      const loaded = loadWasmBuffer(instance, buf);
//      assert.deepStrictEqual(loaded, object);
//
//      wasmBufferFree(instance, buf);
//    });
//  });
//});
//
//describe("Deploy Template", function () {
//  it("Encodes & Decodes valid transactions", function () {
//    return compileWasmCodec().then((instance) => {
//      let tx = {
//        svm_version: 1,
//        code_version: 2,
//        name: "My Template",
//        desc: "A few words",
//        code: "C0DE",
//        data: "0000000100000003",
//        ctors: ["init", "start"],
//      };
//
//      const buf = wasmNewBuffer(instance, tx);
//      const result = instanceCall(instance, "wasm_encode_deploy", buf);
//
//      let len = wasmBufferLength(instance, result);
//      const slice = wasmBufferDataSlice(instance, result, 0, len);
//      assert.strictEqual(slice[0], OK_MARKER);
//
//      // `bytes` is a `Uint8Array` holding the encoded `SVM spawn-account` transaction
//      const bytes = slice.slice(1);
//
//      wasmBufferFree(instance, buf);
//      wasmBufferFree(instance, result);
//    });
//  });
//  it("Handles errors for invalid transactions", function () {
//    return compileWasmCodec().then((instance) => {
//      let tx = {
//        svm_version: 1,
//        code_version: 2,
//      };
//
//      const buf = wasmNewBuffer(instance, tx);
//      const result = instanceCall(instance, "wasm_encode_deploy", buf);
//
//      const error = loadWasmBufferError(instance, result);
//      assert.strictEqual(error, "A non-optional field was missing (`name`).");
//
//      wasmBufferFree(instance, buf);
//      wasmBufferFree(instance, result);
//    });
//  });
//});
//
//describe("Spawn Account", function () {
//  function encodeSpawn(instance, template, name, calldata) {
//    let tx = {
//      version: 0,
//      template: template,
//      name: name,
//      ctor_name: "initialize",
//      calldata: calldata,
//    };
//
//    const buf = wasmNewBuffer(instance, tx);
//    const result = instanceCall(instance, "wasm_encode_spawn", buf);
//
//    let len = wasmBufferLength(instance, result);
//    const slice = wasmBufferDataSlice(instance, result, 0, len);
//    assert.strictEqual(slice[0], OK_MARKER);
//
//    wasmBufferFree(instance, buf);
//    wasmBufferFree(instance, result);
//
//    return slice.slice(1);
//  }
//
//  function decodeSpawn(instance, bytes) {
//    const data = binToString(bytes);
//
//    const buf = wasmNewBuffer(instance, { data: data });
//    const result = instanceCall(instance, "wasm_decode_spawn", buf);
//    const json = loadWasmBufferDataAsJson(instance, result);
//
//    wasmBufferFree(instance, buf);
//    wasmBufferFree(instance, result);
//
//    return json;
//  }
//
//  it("Encodes & Decodes valid transactions", function () {
//    return compileWasmCodec().then((instance) => {
//      const template = generateAddress("1020304050");
//      const name = "My Account";
//
//      const object = {
//        abi: ["i32", "i64"],
//        data: [10, 20],
//      };
//
//      let calldata = encodeInput(instance, object);
//      const bytes = encodeSpawn(instance, template, name, calldata["data"]);
//      const json = decodeSpawn(instance, bytes);
//
//      assert.deepStrictEqual(json, {
//        version: 0,
//        template: template,
//        name: name,
//        ctor_name: "initialize",
//        calldata: {
//          abi: ["i32", "i64"],
//          data: [10, 20],
//        },
//      });
//    });
//  });
//  it("Handles errors for invalid transactions", function () {
//    return compileWasmCodec().then((instance) => {
//      let tx = {
//        version: 0,
//        template: "102030",
//      };
//
//      const buf = wasmNewBuffer(instance, tx);
//      const result = instanceCall(instance, "wasm_encode_spawn", buf);
//
//      const error = loadWasmBufferError(instance, result);
//      assert.strictEqual(
//        error,
//        "The value of a specific field is invalid (`template`)."
//      );
//
//      wasmBufferFree(instance, buf);
//      wasmBufferFree(instance, result);
//    });
//  });
//});
//
//describe("Call Account", function () {
//  function encodeCall(instance, target, verifydata, calldata) {
//    let tx = {
//      version: 0,
//      target: target,
//      func_name: "do_something",
//      verifydata: verifydata,
//      calldata: calldata,
//    };
//
//    const buf = wasmNewBuffer(instance, tx);
//    const result = instanceCall(instance, "wasm_encode_call", buf);
//
//    let len = wasmBufferLength(instance, result);
//    const slice = wasmBufferDataSlice(instance, result, 0, len);
//    assert.strictEqual(slice[0], OK_MARKER);
//
//    wasmBufferFree(instance, buf);
//    wasmBufferFree(instance, result);
//
//    return slice.slice(1);
//  }
//
//  function decodeCall(instance, bytes) {
//    const data = binToString(bytes);
//
//    const buf = wasmNewBuffer(instance, { data: data });
//    const result = instanceCall(instance, "wasm_decode_call", buf);
//    const json = loadWasmBufferDataAsJson(instance, result);
//
//    wasmBufferFree(instance, buf);
//    wasmBufferFree(instance, result);
//
//    return json;
//  }
//
//  it("Encodes & Decodes valid transaction", function () {
//    return compileWasmCodec().then((instance) => {
//      const target = generateAddress("1020304050");
//
//      let verifydata = encodeInput(instance, {
//        abi: ["bool", "i8"],
//        data: [true, 5],
//      });
//
//      let calldata = encodeInput(instance, {
//        abi: ["i32", "i64"],
//        data: [10, 20],
//      });
//
//      const bytes = encodeCall(
//        instance,
//        target,
//        verifydata["data"],
//        calldata["data"]
//      );
//      const json = decodeCall(instance, bytes);
//
//      assert.deepStrictEqual(json, {
//        version: 0,
//        target: target,
//        func_name: "do_something",
//        verifydata: {
//          abi: ["bool", "i8"],
//          data: [true, 5],
//        },
//        calldata: {
//          abi: ["i32", "i64"],
//          data: [10, 20],
//        },
//      });
//    });
//  });
//  it("Handles errors for invalid transactions", function () {
//    return compileWasmCodec().then((instance) => {
//      let tx = { version: 0, target: "102030" };
//
//      const buf = wasmNewBuffer(instance, tx);
//      const result = instanceCall(instance, "wasm_encode_call", buf);
//
//      const error = loadWasmBufferError(instance, result);
//      assert.strictEqual(
//        error,
//        "The value of a specific field is invalid (`target`)."
//      );
//
//      wasmBufferFree(instance, buf);
//      wasmBufferFree(instance, result);
//    });
//  });
//});
//