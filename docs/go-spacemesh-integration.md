# Integration with `go-spacemesh` plan
This document is intended to serve as a high-level plan for integration of SVM with [`go-spacemesh`][go-spacemesh].

There are two main purposes for this doc:

* Make sure that spacemesh team members are aligned with the high-level plan and terminology.
* Form a basis from which GitHub issues will be created.

<br/>

Some notes:

* SVM is using [`wasmer`][wasm] as its underlying WebAssemby Runtime. 
<br/>
Other wasm runtimes could (theoretically) be used instead.
<br/>
However, at this point in time `wasmer` seems the best fit for us.
<br/>
In the future we could add support for other WebAssembly Runtimes.
<br/>
(see also: [`wasmtime`][wasmtime] by _Mozilla_).
<br/>
If you'll do that, we'll have to make sure that we reach consensus while part of the nodes run `wasmer` and other `wasmtime`.
<br/>
(otherwise, we'll have to stick for one wasm runtime).

* Since SVM is a standalone project this document may be a good reference for any other future Blockchain projects willing to integrate SVM.
<br/>
(theoretically any programming-language supporting FFI interface).

<br/>

## Terminology

#### `go-svm`
Golang client for SVM.
The client will interface with the `svm.h` and `svm` object file using `cgo`. (SVM has an _FFI_ interface).
<br/>
It will expose a Golang idiomatic interface and be used as a dependency within the `go-spacemesh` project.

#### `Host`
WebAssembly (wasm) programs are running within a restricted environment (sandbox) for security reasons.
<br/>
The entity running the VM is called the `Host`. In our case the `Host` will is `go-spacemesh`.

#### `Host Imports`
Web-Assembly programs without any interface to the external-world (i.e the `Host`) aren't capable to achieve much. (rendering them stateless too).
That's why any wasm program can import functions (and other things - not relevant for this document) and invoke them.
</br></br>
The classic use-case is a wasm program calling the `Host`'s `get_balance`. 
</br>
Another one is `transfer` (moving coins between two accounts).

#### `Transaction Envelope`
This term refers to any transaction data besides SVM specific data.
It will be mentioned usually in the context of transaction fields such as: `sender`, `value`, `gas_limit`, `gas_price` and `nonce`.

#### `Host Context`
This term refers to the context of the `Host`. Meaning, the data of `Transaction Envelope` + additional data.
It will contain fields such as: `block_id`, `layer_id` (and fields of `Transaction Envelope`).
<br/>
Executed SVM transactions will have access to the `Host Context`.
<br/>
The data-structure used for `Host Context` will be a Map between an u32 integer index to a raw-byte array.

Here is an example:
<br/>
``` 
{
	// `0` denotes `nonce` in this example.
	0 => [0x10, 0x20],
	
	// `1' denotes `layer_id` in this example. 
	1 => [0x1A, 0x5C, 0x2D]

    // ...
}
```

#### `App Template` 
We name a `Smart Contract`'s code + metadata (including storage spec) as a `App Template`.
We can think of `App Template` as the equivalent of `class` in an Object-Oriented programming paradigm.
<br/>
Each `Template` will have an account under the `Global State` and its own `Addres`. (see more under `Global State` section).


#### `App` 
Given an `App Template` - we can spawn `App`s out of it.
<br/>
All spawned `App`s out of the same source `Template` share the same code but have an isolated inner state. 
We can think of an `App` as the equivalent of a `class instance` (a.k.a `object`) under the Object-Oriented programming paradigm.
<br/>
The motivation for having both `App Template` and `App` are encouraging code reuse and saving of on-chain storage.
Each `App` will have an account under the `Global State` and its own `Address`. (see more under `Global State` section).


#### `App Transaction`
Given a spawned `App` - we'd like to execute `App Transaction`s on it.
<br/>
We can think of executing an `App Transaction` as the equivalent of invoking an `object method` in an Object-Oriented programming paradigm.
<br/>
Executing `App Transaction` are the way to apply changes and consequently transition the state of an `App`. 


#### `App State` 
Hash referencing the current `State` of an `App`. The internal data of each `App` is managed internally by SVM. 
<br/>
The Receipt of a successful `Exec App (a.k.a calling method)` transaction will include the new `App State`.
<br/>
See also: `App Account` under `Global State`.


#### `SMESH language`
This term refers to a future programming-language code-named `SMESH`.
<br/>
This first version will have no loops (maybe only bounded-iterations loops) and more restrictions.
<br/>
The language program will compile to a plain Spacemesh-flavored wasm.
<br/>
And the target audience will be people with basic programming skills.
The motivation is having a low entry barrier for coding templates using `SMESH`.

The integration of SVM within `go-spacemesh` is a prerequisite for starting the work on `SMESH`.

<br/>

## High-level flows

SVM orchestrates 3 kinds of transactions. Each transaction returns a Receipt that will be persisted on-chain.
<br/>
(see also `Raw Transactions format` and `Receipts` sections).

#### `Deploy App Template`
The `go-spacemesh` v0.2 will contain only a single built-in template, named `Smart Wallet`.
<br/>
Therefore, the `deploy-template` functionality using the `p2p` should be disabled.
<br/>
See `Genesis flow` for how to deploy the pre-built `Smart Wallet`.
<br/>


#### `Spawn App`
The `go-spacemesh` v0.2 will support only apps of the `Smart Wallet` template.
<br/>
Part of the apps will be spawned as part of the `Genesis flow` and the rest apps will be spawned via the `Wallet UX` client. 
<br/>

The steps:

1. Wallet UX user picks the required template. For `go-spacemesh` v0.2 the template will always be the `Smart Wallet`.
1. The `spawn app` interface is displayed with constructor input fields derived from the `App Template ABI`.

   Special attention should be given to the `value` field, which is part of the `Transaction Envelope`.
   The balance of the `spawned-app` will be initialized with `value`. (it will be transferred from the app's creator balance).
   
1. User fills-in the constructor fields.
1. The estimated required `gas_limit` is shown to the user.
1. If the end-user (app creator) has enough balance also for the `gas_limit` he may click the `Spawn App` button.
1. Clicking the `Spawn App` button will dispatch the `Spawn App` transaction to the network.


#### `Execute App Transaction (a.k.a Call Method Transaction)`
The steps:

1. Wallet UX user picks the desired app. The user needs to have its `Address`.
1. The `execute app` interface is displayed by showing the public API methods of the `App`.
1. User selects the desired API method.
1. User fills-in the method fields.

   Special attention should be given to the `value` field, which is part of the `Transaction Envelope`.
   Amount of `value` will be transferred to the `App`. (transferred from the `sender`'s balance).
   
1. The estimated required `gas_limit` is shown to the user.
1. If user (a.k.a `sender`) has enough balance also for the `gas_limit` he may click the `Execute App` button.
1. Clicking on the `Execute App` button will dispatch the `Execute App` transaction to the network.


#### `Reading App's Storage`
The steps:

1. Wallet UX user picks the desired app. This user needs to have its `Address`.
1. The `App State ABI` is downloaded and rendered to the user. (off-chain data).
1. Wallet UX invokes a batch call asking for each `App Storage`'s field. 
<br/>
The ABI will be further developed with data-structures added to `SVM` storage.
<br/>
For now, only fixed-size fields will be supported: (uint32, bool, `Address`, etc).

## Validation (Mempool)
Each network-peer should perform syntactic validation to SVM transactions. 
<br/>
In case the validation fails, the transaction should be discarded.

## Mining - Which transactions to pick?
The miner will have to decide which transactions are most appealing to him.
<br/>
Also, we want to have a minimum overlap between miners selections.
<br/>
The issue with Smart-Contracts is that we only have gas estimation which derives the `gas_limit`.
<br/>
However, since we only allow a restricted-set of WebAssembly having no loops we can achieve a better estimation.
<br/>
The total gas estimation will consist of 2 parts:

* Execution estimation 
* Payload size - This is a number we can know exactly ahead. 
* Storage size  - We can know-ahead the root-hierarchy size (it's specified in the `App Template` spec). 
<br/>
Talk about the algorithm: see under the `Open Questions` section.

## Raw Transactions format
We'll need to introduce a transaction type flag to the `Transaction Envelope`
<br/>
For example:

* type=0  simple transaction with `SVM-ed25519`
* type=1  simple transaction without standard signatures (supported by Hardware Wallets).
* type=2  deploy a template. SVM 0.2 should disable that type.

```
+-----------------------------+
|   Transaction Envelope      |
+-----------------------------+
| type=2 |   deploy-blob      |
+-----------------------------+
```

The `deploy-template` blob layout can be read here:

https://github.com/spacemeshos/svm/blob/master/crates/svm-app/src/raw/template/mod.rs#L1


* type=3  spawn app

```
+-----------------------------+
|   Transaction Envelope      |
+-----------------------------+
| type=3 |  spawn-app blob    |
+-----------------------------+
```

The `spawn-app` blob layout can be read here:

https://github.com/spacemeshos/svm/blob/master/crates/svm-app/src/raw/app/mod.rs#L1



* type=4  execute-app (call method)

```
+-----------------------------+
|   Transaction Envelope      |
+-----------------------------+
| type=4 |  exec-app blob     |
+-----------------------------+
```

The `exec-app` blob layout can be read here:

https://github.com/spacemeshos/svm/blob/master/crates/svm-app/src/raw/transaction/mod.rs#L1


## Global State

SVM requires two new account types to be added:

#### `App Template Account`
After deploying a template successfully, a new account of type `Template` should be added to the `Global State`.
<br/>
The `state` of this account should be set to zeros (it's meaningless).
<br/>
The `balance` of this account should be set to zero.
<br/>
Sending coins to the `Template` account in any future transaction will lock these for good. 

#### `App Account`
After spawning `App` successfully, a new account of type `App` should be added to the `Global State`.
<br/>
The `App`'s initial `state` is returned by the `Spawn Receipt` (see more data under `Receipts` section).
<br/>
The `balance` of this account should be set with the `value` given by the `Spawn App` transaction sender.

The data for an `App` account will be:

* `Address`   - Same as any `Global State` account.
* `Balance`   - Same as any `Global State` account.
* `App State` - See the `App State` under `Terminology` section.

<br/>
Optional (requires discussion):

* `Creator` - The address of the `App` spawner.
* `Author`  - The address of the `App Template` author.

#### Committing changes
While executing an `App Transaction`, the app will commit changes to the App's storage and to the balances of accounts.
<br/>
Upon a successful transaction, SVM will persist the `App storage` changes and re-calculate a new `State`. 
<br/>
Then, the `Receipt` will include that new `State`. 

Now, the `Global State` should:

* Update the `App` leaf-node with the new App's `State`.
* Play the dirty coins transfers.
* Recalculate the new Merkle-Tree Hashes.


## Receipts
There are 3 types of `Receipt`s: `deploy-template`, `spawn-app` and `exec-app`.
<br/>
Each `Receipt` should be persisted on-chain in its raw packed form.
<br/>
Additionally, `SVM` exposes `Receipt helper methods` for extracting each field in isolation.
<br/><br/>
#### `Deploy App-Template` 
If the `is_success` field if `true` it means that the `deploy-transaction` has succeeded.
<br/>
Then, the `template_address` should be extracted (from the receipt) for the new `App Template` account creation. 
<br/>
(see `Global State` section).

Fields:

* `is_success = true`
* `template_address` 
* `gas_used`

When `is_success` is _false_ - it means that the `deploy-transaction` has failed.
Now, `go-spacemesh` needs to fee the `sender` with the `gas_limit`.
Both `sender` and `gas_limit` fields are sent as part of the `Transaction Envelope`.
<br/><br/>
#### `Spawn App`
When the spawned-app succeeds (`is_success = true`) the returned receipt contains the following:

* `app_address` - The `address` of the spawned-app.
* `init_state`  - The initial `state` of the `App` (after executing the constructor).
* `returns`     - The executed function returned values. Array of `wasm value`. Each value is `i32` or `i64`.
* `gas_used`    - The amount of gas used.
<br/><br/>
#### `Execute App-Transaction` 
When the executed app-transaction succeeds (`is_success = true`) the returned receipt contains the following:

* `new_state` - The new `state` of the `App`
* `returns`   - The executed function returned values. Array of `wasm value`. Each value is `i32` or `i64`.
* `gas_used`  - The amount of gas used.


## On-Chain data

* Each transaction should be part of `Transactions Tree.`
* `App Template` and `App` accounts will be part of the `Global State`.
* `Receipt` should be on-chain too.
* SVM manages the data of each `App` and provides the `App State` to the `Global State`.


## Genesis flow
As mentioned above, `go-spacemesh` v0.2 will come with a single built-in template, named `Smart-Wallet`.
<br/>
Let's mark the folder as `src/apps/smart-wallet` and the `App Template` raw data as `src/apps/smart-wallet/deploy.bin`.
<br/><br/>
The Genesis flow will invoke SVM Runtime `Deploy Template` (using the `go-svm` client) method.
<br/>
The `Host Context` fields that are sent over-the-wire will have to be filled in, since there will be no real
p2p `deploy template` transaction of the `Smart Wallet` template.
<br/><br/>
If the deployment of the `Smart Wallet` fails (theoretically) - the whole `Genesis flow` should halt.
<br/>
Now, given a successful deployment, we need to manually create a single account containing the minted coins of _Spacemesh_. 
<br/>
Let's denote this account address as `MINT`.
<br/><br/>
Next, we need to iterate over a configuration file containing all the so-called "investors". 
<br/>
For each "investor" we'll spawn a `Smart Wallet App`. The app-spawner (transaction `sender`) will be `MINT`.
<br/><br/>
The `value` field of the spawn transaction will be the `coins` field (see the configuration file).
It means that `value` coins will be transferred from `MINT` account to the new `App` account.
<br/>
If the spawning a `Smart Wallet` fails (theoretically) - the whole `Genesis flow` should halt.
<br/>

Gas concerns: during the `Genesis flow` the `gas_metering` flag will be turned-off. (SVM supports that).
<br/>

Here is an example of how the configuration file may look like.
<br/>
Regarding the `nickname` field - see more under the `Name-Service` section.

```
{
  "investors": [
    {
	  "nickname": "@tons-of-coins",   // Not in SVM v0.2 - see more under the `Name-Service` section.
	  "is_multisig": true,            // MultiSig turned-on 
	  "pub_keys": [..],               // An array with `3` public-keys since `is_multisig=true`
	  "total_coins": 10000,           // The number of coins that will be eventually vested.
	  "vesting_months": 48,           // 4-years vesting.
	  "lockup_months":  12,           // The wallet will be locked for 12 months. 

	  // more params
	},
	{
	  "nickname": "@not-many-coins",  // Not in SVM v.0,2 - see more under the `Name-Service` section.
	  "is_multisig": false,           // MultiiSig turned-off
	  "pub_keys": [..],               // An array having a single public-key since `is_multisig=false`.
	  "total_coins": 100,             // The number of coins that will be eventually vested.
	  "vesting_months": 48,           // 4-years vesting.
	  "lockup_months":  12,           // The wallet will be locked for 12 months. 

	  // more params
	},
	...
  ]
}
```



## App Storage Read ABI
TBD


## Open Questions

* What Hash algorithm to use for SVM - should it be _BLAKE3_ ?
* Algorithm for deciding which transactions a miner should pick.
* Signatures Scheme.
* `Receipt` should be part of the `Transactions Mesh` or in other data-structure? 
* `Balance` representation. Can we use a single `i64` or a pair of `i64`? 
* Decide on the exact formula for deriving the `Template` and `App` accounts addresses.
* Does the `returns` field of the `Spawn App` and `Exec App` Receipts should be discarded?
  The size-volume of the field won't affect the final `gas_used`... 
* What will be the `gas_price` injected into a transaction? 
* We need to figure out what indexes should be created in `go-spacemesh` for assisting the _Transactions Explorer_.
  
  Examples for such indexes.
  ```
  tx_id    -> Receipt
  layer_id -> [Receipt]
  ```

* Do we want to have the encoding prefix of each Receipt kind to be the same?
  ```
  (version, receipt_type, is_success, gas_used)
  ```

<br/>

## Out-of-scope for SVM 0.2
Here is the list of things that won't be included in SVM 0.2 but must be in the subsequent 0.3 version.

#### Generic Call Method ABI 
Needs more research.

#### Transient Events
We'd like SVM to emit events that will be persisted for an ephemeral amount of time.
<br/>
By having transient events, we can avoid the feature abuse done on other chains.
<br/>
This capability should become very useful for debugging and the transaction Explorer.
<br/>
The events won't be part of a Receipt. 

#### Name-Service
We may want to be able to correlate each "investor" wallet App's `Address` with a nickname.
<br/>
Implementing a _Name-Service_ `App Template` will enable us to do that.
<br/>
Then, we can include the nickname as part of the configuration file. 
<br/>
For more info, see the `Genesis flow` section.

#### Avoiding Template Duplication 
Currently, raw deployed templates will be duplicated. 
<br/>
Once the data will be saved on-mesh as its for any transaction. and the second time, as internal of SVM.
<br/>
We need to decide before Mainnet whether we want to go the extra mile and save the template raw data only once.
<br/>
This decision is may some storage but make the `SYNC` process slower and add maintenance costs to `go-spacemesh`. 



[go-spacemesh]: https://github.com/spacemeshos/go-spacemesh
[wasmer]: https://wasmer.io/
[wasmtime]: https://wasmtime.dev/
