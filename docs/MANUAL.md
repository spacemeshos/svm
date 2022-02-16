# The SVM manual

The SVM (Spacemesh Virtual Machine) is the engine that drives smart contracts execution inside `go-spacemesh`. It is written in Rust and communicates with the node -which is written in Golang- using FFI.

## Notes about architecture

The only way to interact with the SVM is with a **runtime**. An SVM runtime exposes an API for all supported operations and is backed by a single SQLite instance. The FFI adopted by `go-spacemesh` is but a thin wrapper around the Rust interface to the SVM runtime. 

### Motivation

The SVM component exists to serve as the transaction executor.
Each transaction selected to execute by the Full-Node is passed to SVM that runs it and returns a receipt as a result.
In addition to running transactions, SVM exposes a validation interface. Thus, before running a transaction, we need to make sure that it's syntactically valid.

The other primary role of SVM is owning the so-called `Global-State`

The `Global-State` is a fancy name for `The Bank of All System Accounts`(More information later under the `Global State` section).
When a transaction runs, it runs on-behalf of an Account (this is not precisely always the case - see `Transactions Types` later).

The transaction running code will run and possibly create changes to the executing Account.
In other words, running a transaction isn't a stateless process. One of the implications is that the order of running transactions matters.
Each transaction will start running given a contextual `State` and finish with a new `State`.

Sometimes the ending `State` will be the same as the starting one. For example, it might be due to running code that didn't modify any account in the system.
Other times, the transaction didn't have enough gas to execute fully, and any uncommitted changes will be discarded.
The `State` of the system will remain the same, and the Account sending the transaction (the principal) will have to pay for the failed running.

### High-level Description

- **Runtime**
  The Runtime is the piece that orchestrates both validation and execution of transactions.
  It integrates with a Wasm Runtime (we currently use [Wasmer](https://wasmer.io/)) and makes sure each running transaction returns a receipt.
- **Accounts**
  In a very similar way to bank accounts in real life, in SVM, there're more general accounts.
  An account is very similar to an actor under the [Actor Model](https://en.wikipedia.org/wiki/Actor_model) design or an object in Object-Oriented languages.

Each Account essentially has its code and its internal data - we'll refer to it as its storage.
Two different accounts can hold the same code, but each will have its private storage.

- **Templates**
  To encourage code reuse and saving of used disk space, SVM introduced the notion of a Template.
  A Template is a recipe from which accounts are spawned. Each `Template` might be used to spawn many accounts.
  Each spawned Account will be attached to the same Template, but the code will be stored only once.
- **Transactions Types**
  The SVM comes with three types of transactions:

- **Deploy**
  To leverage a Template, we first need to make sure it's deployed.
  Each Template has a unique `Template Address` assigned to it.

For the 1st Mainnet - each Template deployment will take place as part of the Genesis flow.

- **Spawn**
  Once we've got a deployed `Template`, we can start to exploit it and spawn many accounts of it.
  As part of running a `Spawn` transaction, the new Account gets to execute a constructor function.
  Running constructors will be (mostly) used for initializing the new account storage.

- **Call**
  After we've spawned a new account, we can start using it. That's the job of the `Call` transaction.
  The `Call` transaction is by far the most used transaction type on the system since spawning accounts occur much less often.

- **Codec**
  The `Codec` is responsible for encoding & decoding transactions and receipts.
  Its code is shipped on two platforms. One is SVM, and the other is a Wasm package to be used by clients such as `smapp`
  The reason for shipping the `Codec` in its Wasm form is that clients will be able to craft transactions before dispatching them to the network.
  Additionally, clients and others (the Process Explorer) would like to decode historical transactions and receipts.
  They can use the `Codec` Wasm package for doing that.

- **Global-State**
  The Global-State is in charge of all accounts. Therefore, it can do the basics such as creating accounts or transferring coins between accounts, but it always operates in the context of a given State.
  The Global-State may be requested to rewind to a historical `State`.
  Each running accumulates dirty changes that will be persisted on the next commit. As long as changes have not been committed, the Global-State can be asked to drop them.
  It's done when a transaction fails (panics or reaches Out-Of-Gas) - in that case; the Global-State will discard its dirty changes.

- **SDK**
  The SDK of the SVM project exists to assist with developing Templates in Rust.
  In theory, Templates can be manually created by writing Wasm, but it's not a feasible solution.
  The other alternative is writing code in a high-level language that compiles to Wasm, but doing that without some SDK will be unwieldy.
  Besides making lives more manageable for the developer of a Template, the SDK's job is to make sure that the emitted Wasm code adheres to the restrictions of a Fixed-Gas Wasm.
- **Fixed-Gas**

  Each transaction becoming part of the Mesh needs to pay gas for its execution.
  The problem is that we don't have any good answer to the question "How much gas will this transaction require?".
  The reason for that is rooted in the [Halting Problem](https://en.wikipedia.org/wiki/Halting_problem). Given a program, we can't know whether or not it will ever halt.
  The implication is that SVM can't execute a single transaction without setting a gas limit for running a transaction.

The Fixed-Gas solution tries to find a sweet point between a good gas estimation usage experience and what a running program can achieve while running.
The solution is that if a program has no loops - we can give a gas estimation that will guarantee we won't hit Out-Of-Gas.
Loops must not exist in any form—neither explicit Wasm loop nor recursions nor cyclic calls between functions.

For more information, watch this video:
[https://www.youtube.com/watch?v=nx7Na1Kf21A](https://www.youtube.com/watch?v=nx7Na1Kf21A)

At this point, SVM doesn't support Gas-Metering for Wasm programs having loops. As a result, such programs will fail validations today for not being Fixed-Gas compliant.

- **FFI**
  SVM ships with an FFI layer to be used by non-Rust code.
  Since the Node hosting SVM is written in Golang (see: [https://github.com/spacemeshos/go-spacemesh](https://github.com/spacemeshos/go-spacemesh)), the FFI layer is essential.

Golang has the `cgo` package that makes calling C code possible.
To make life easier for the `go-spacemesh` project, we've created a library called `go-svm`(link: [https://github.com/spacemeshos/go-svm](https://github.com/spacemeshos/go-svm))
that does the heavy lifting of using `cgo` against the SVM objects files. The end-user of `go-svm` should enjoy an excellent ergonomic Golang API.

### Guidelines

- **Fast**
  It's crucial that SVM will execute as fast as we can.
  Besides being written in Rust, the Singlepass compiler will be used (Windows solutions is upcoming on Wasmer's next release).

- **Secure**
  The transactions are running in a sandboxed environment (it's one of Wasm's most significant strengths).
  Besides that, using the Singlepass solution guarantees a linear-time compilation and defense against JIT Bombs.

- **Portable**
  Since Spacemesh Full-Node is targeted for the mass, it must work on any of the three popular Operating-Systems: macOS, Linux, and Windows.


## A quick tour

Most of the SVM code sits inside `crates/`, where it's partitioned in two groups of Rust crates:

- Crates that define tooling, some testing infrastructure and most of the SVM implementation. All of this code is designed to run on the host side, i.e. natively (x86, ARM). These crates sit under `crates/host/`.
- Crates that compile down to WebAssembly, the code of which is intented to run inside the actual SVM. These crates sit under `crates/wasm/`.

---

`crates/host/runtime` is the entry point of the project. It brings together (almost) all the other crates, importing them either directly or undirectly, and defines a straightforward Rust API to run smart contracts inside the SVM. `crates/host/runtime-ffi` works similarly, but with FFI.

---

`crates/host/query/` is outdated. It can and should be removed. It's not even used
anywhere inside the project, so the removal is as easy as deleting a directory.

---

`crates/host/types/` defines some common types used throughout the whole SVM project. Here's some examples:

- `Account`- represents an account in the system.
- `Address` - the address of an account.
- `Receipt types` - each transaction type has a Receipt type.
- `Template` - a reusable definition of a smart contract. Each template is made up by many "sections". Some of these are mandatory (like `CodeSection`, i.e. the WebAssembly code of the smart contract), others are optional (like `SchemaSection`, which documents on-chain how to interact with the contract with 3rd party calls).
- `Envelope` - for the transactions' envelope.
- `Context` - for the transaction's executing context.
- `Transaction` - contains the `Call Message` data.

---

`crates/host/state` encapsulates logic around disk persistence of all SVM data. This includes account balances, account nonces, account metadata, template code, other template sections, etc.. After each successful smart contract execution, users can *checkpoint* and finally *commit* at layer finalization. Upon commitment, a "fingerprint" is returned, which serves the same purpose as e.g. Bitcoin and Ethereum's root signature. Please note that it's not an actual cryptographic signature, so the current design does *not* accomodate for light clients and Merkle proofs.

---

`crates/host/gas` and `crates/host/program` perform smart contract code verification.

`svm-gas` performs, like the name suggests, gas-related verification tasks:

- Given a Wasm program, `svm-gas` determined whether Fixed-Gas compliant or not.
- Given a Fixed-Gas compliant Wasm program, `svm-gas` computes its gas price for each function. Gas price of each function is defined as the cost of its most expensive execution path. This calculation relies on call graph static analysis.

`svm-program` performs all other forms of static analysis, validation, and inspection over WebAssembly code that the SVM is interested in.

Code that is fully validated by both `svm-gas` and `svm-program` is called smWasm (Spacemesh WebAssembly). Please note that smWasm is a strict subset of WebAssembly.

---

`crates/host/codec` defines a compact encoding for all SVM-related data that needs to be either stored on disk or sent via wire to other nodes. Please note that different entities **must** map to different byte sequences, and different byte sequences **must** map to different entities.

The only notable entities that adopt `crates/host/codec` are:

1. Templates' sections (stored on disk).
2. Smart contract execution receipts (sent to other nodes).

This crate exposes three APIs:

- Rust API - used internally by SVM.
- Wasm API - for use by blockchain explorers, Spacemesh node GUI, etc.. Available as a CI artifact named `svm_codec.wasm` for every Git-tagged release of SVM.
- JSON API - closely related to the Wasm API. Defines a direct mapping between  encoded data and JSON objects.

---

`svm-layout` implements layout specification of templates' persistent storage - as in, what variables a template stores and what their sizes are.

---

`crates/wasm/abi-*` defines the default SVM ABI. Unlike the EVM, SVM does *not* mandate the usage of a single ABI and smart contract authors can define their own ABIs. Note that ABIs effectively define the calling convention for interacting with smart contracts, so they are subject to space requirements as well as UX considerations. The default SVM ABI is compact, well-documented, and supports a lot of common types e.g. integers, booleans, account addresses, etc..

Usages:

- Encoding of functions inputs (e.g., `calldata`, `verifydata`)
- Decoding the functions returns (e.g. `returndata`

---

`crates/wasm/sdk` makes it easy to write SVM templates using Rust that will compile to SVM-compliant WebAssembly modules. It's deeply integrated with the default SVM ABI (`crates/wasm/abi-*`).

- `svm-sdk-alloc`. Implements dynamic allocation using a straightforward "bump allocator".
- `svm-sdk-std`. Defines common types and `std`-like components that are Fixed-Gas compliant.
- `svm-sdk-types`. Defines some more common types. Fixed-Gas compliant, just like `svm-sdk-std`.
- `svm-sdk-storage`. Implements an API that exposes smart contract storage getters and setters via host calls.
- `svm-sdk-host`. Similarly to `svm-sdk-storage`, it introduces an API that exposes functionalities provide by the host via host calls.
- `svm-sdk-macros`. Rust procedural macros to define SVM templates.
- `svm-sdk`. The SDK prelude crate. Glues together all the other SDK crates.

Please note that mocks are available for some of these crates; although such mock crates are found inside `crates/wasm/`, they technically don't target WebAssembly because basic testing of smart contracts happens by entirely mocking all host calls. **This is not a long-term solution. Tests should spin up separate instances of the SVM `Runtime`.**

---

`crates/host/cli` is special in the sense that it's not part of the actual SVM, but rather it's a complementary tool that is designed to assist smart contract authors. It integrates deeply with the SDK (e.g. smart contract "schema data" output by the SDK can be processed by this CLI). It can take JSON descriptions of transactions and output their binary representation; it also exposes Fixed-Gas validation logic.
