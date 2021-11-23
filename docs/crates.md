# Crates High-level review

## Types

The `svm-types` crate contains types that are used on multiple crates.

A few types are defined:

- `Account`- represents an account in the system.
- `Address` - an address of an account.
- `Receipt types` - each transaction type has a Receipt type.
- `Template` - represents a Template (a collection of sections).
- `Envelope` - for the transactions' envelope.
- `Context` - for the transaction's executing context.
- `Transaction` - contains the `Call Message` data.

<br>

## ABI

Implements the SVM's default ABI. From SVM's perspective, each Template is eligible to use a different ABI.
Furthermore, each Template's function can pick a different ABI. It's a best practice to have one ABI per Template.

There is another concern which is crafting transactions. Right now, the `svm_codec.wasm` exposes only the default ABI.
Each ABI used in the future should expose itself via the `svm_codec.wasm` as well.

Usages:

- Encoding of functions inputs (e.g., `calldata`, `verifydata`)
- Decoding the functions returns (e.g. `returndata`

Crates involved:

- `svm-abi-layout`
  This crate is shared between `svm-abi-encoder` and `svm-abi-decoder`
  It contains the encoding of primitive types.
- `svm-abi-decoder`
  The crate implements decoding of encoded data using the default SVM ABI.
  Its logic is used inside the `SVM SDK` (for decoding functions' inputs) and in the `svm-codec` (decoding the `Receipts` returns)
- `svm-abi-encoder`
  The crate implements the encoding of data using the default SVM ABI.
  Its logic is used inside the `SVM SDK`(for encoding functions' inputs) and in the `svm-codec` (encoding the functions returns - which will be returned as part of the Receipts).

<br>

## Codec

The `svm-codec` crate is responsible for:

- Encoding & Decoding of Transactions Messages
- Encoding & Decoding of Receipts

The crate supplies two APIs:

- Wasm API - can be accessed when the crate is compiled to Wasm.
  Each successful CI build of SVM emits an artifact named `svm_codec.wasm`
- JSON API - Its logic is being used by the Wasm API as well.

<br>

## Gas

The `svm-gas` implements these aspects of the `Fixed-Gas Wasm`:

- Validation - given a Wasm program, it determined whether it's a valid Fixed-Gas Wasm or not.
- Pricing - given a Wasm program that has passed validation, It computes its gas price for each function (in terms of gas units).
  A function's gas price is the cost of its most expensive execution path.

<br>

## Layout

Each `Template` under SVM contains:

- Code - The Wasm code of the program
- Layout - The specification of the program's persistent storage.

The `svm-layout` creates implements the layout specification of a Template's persistent storage.
Its code is required in multiple places throughout the SVM project.

<br>

## Program

This `svm-program` crate contains:

- Abstractions for representing a Wasm program and scanning it.
- It contains basic validations, such as checking for invalid opcodes.
- The `svm-gas` crate uses `svm-program` for implementing validation and pricing.
- The crate is also being used by `svm-runtime` crate for obtaining information about the running Wasm program.

<br>

## State

This crate implements the so-called Global-State. It manages all Accounts and Templates living in the system.

<br>

## Runtime

The `svm-runtime` crate is responsible for the execution of transactions and returning Receipts.
It acts as an orchestrator, and it leverages almost any other crate in the workspace.

<br>

## Runtime-FFI

The `svm-runtime-ffi` crate exposes the functionality of `svm-runtime` as an FFI interface.
The `go-svm` projects interfaces against the objects files emitted by the `svm-runtime-ffi` crate upon each successful SVM CI run.

<br>

## SDK

The role of the SDK is to make it accessible to write Templates in Rust that will compile to a Fixed-Gas Wasm.

- `svm-sdk-alloc`
  Implements static-allocation. It's part of the Fixed-Gas Wasm solution. Each allocation takes `O(1)` time and space.
- `svm-sdk-std`
  Implements a tiny std-like functionality that's known to conform to the Fixed-Gas Wasm rules.
- `svm-sdk-types`
  Contains common types to be used within other SDK crates
- `svm-sdk-storage`
  Implements an API that exposes the storage host functions given by the Host.
  Implements both Mock and FFI interfaces. (which is compiled to Wasm import functions behind the scenes).
- `svm-sdk-host`
  Similar to the `svm-sdk-storage` crate, it introduces an API that exposes functionalities given by the Host.
  Implements both Mock and FFI interfaces. (which is compiled to Wasm import functions behind the scenes).
- `svm-sdk-macros`
  Implements Rust Procedural Macros used when writing Templates
- `svm-sdk`
  The SDK prelude crate. Glues together the other SDK crates.

<br>

## CLI

The `svm-cli` crate delivers a CLI that assists primarily with crafting transactions.
It also exposes gas validation & pricing commands.
Future commands are expected to be added in the future.
