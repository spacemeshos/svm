## Integration with `go-spacemesh` plan
This document is intended to serve as high-level plan for integration of SVM with [`go-spacemesh`][go-spacemesh].

There are two main purposes for this doc:

* Make sure that spacemesh team members are aligned with the high-level plan and terminology.
* Form a basis from which GitHub issues will be created.

<br/>
Note: since SVM is a standalone project this document may be a good reference for any other future Blockchain project willing to integrate SVM.


### Terminology

#### `Transaction Envelope`
This term refers to any transaction data besides SVM specific data.
It will be mentioned usually in the context of transaction fields such as: `sender`, `value`, `gas_limit`, `gas_price` and `nonce`.

#### `Host Context`
This term refers to the context of the host. Meaning, the data of `Transaction Envelope` plus extra data.
It will contain fields such as: `block_id`, `layer_id`..

Executed SVM transactions will have access to the `Host Context`.

The data-structure used for the `Host Context` will be a Map between an i32 integer index to a raw-byte array.

``` 
{
	# `0` denotes `nonce` in this example.
	0 => [0x10, 0x20],
	
	# `1' denotes `layer_id` in this example. 
	1 => [0x1A, 0x5C, 0x2D]
}
```

#### `App Template` 
We name a `Smart Contract`'s code + metadata (including storage spec) as a `App Template`.
We can think of a `Template` as the equivalent of a `class` in an Object-Oriented programing paradigm.
<br/>
Each `Template` will have an account under the `Global State` and its own `Addres`. (see more under the `Global State` section).

#### `App` 
Given an `App Template` we can spawn `App`s out of it.
All spawned `App`s out-of the same origin `Template` share the same code but have an isloated inner state. 
We can think of an `App` as the equivalent of a `class instance` (a.k.a `object`) in an Object-Oriented programing paradigm.
<br/>
The motivation for having both `App Template` and `App` are encouraging code reuse and saving of on-chain storage.
Each `App` will have an account under the `Global State` and its own `Address`. (see more under the `Global State` section).

#### `App-Transaction`
Given a spawned `App` we'd like to execute `App Transaction`s on it.
<br/>
We can think of executing an `App Transaction` as the equivalent of a invoking an `object method` in an Object-Oriented programing paradigm.
<br/>
Executing `App Transaction` are the way to apply changes and transaction the state of an `App`. 


#### `App-State` 
Hash referencing the current `State` of an `App`. The internal data of each `App` is managed internally by SVM. 
The Receipt of a successful `Exec App (call method)` transaction will include the new `App State`.
See also: `App Account` under `Global State`.


### High-level flows

SVM orchestrates 3 kinds of transactions. Each transaction returns a Receipt that will be persisted on-chain.
(see also `Raw Transactions format` and `Receipts` sections).

#### `Deploy App Template`
The `go-spacemesh` v0.2 will contain only a single built-in template, named `MultiSig Wallet`.
Therefore, the `deploy-template` functionality using the `p2p` should be disabled.

See `Genesis flow` for how to deploy the pre-built `MultiSig Wallet`.
<br/>
#### `Spawn App`
The `go-spacemesh` v0.2 will support only apps of the `MultiSig Wallet` template.
Part of the apps will be spawned as part of the `Genesis flow` and the rest apps will be spawned via the `Wallet UX` client. 
<br/>

The steps:

1. Wallet UX user picks the required template. For `go-spacemesh` v0.2 the template will always be the `MultiSig Wallet`.
1. The `spawn app` interface is displayed with constructor input fields derived from the `App Template ABI`.

   Special attention should be given to the `value` field, which is part of the `Transaction Envelope`.
   The balance of the `spawned-app` will be initialized with `value`. (it will be transfered from the app's creator balance).
   
   TOD: how to derive the `gas_price` ?
   
1. User fills-in the constructor fields.
1. The estimated required `gas_limit` is shown to the user.
1. If user (app creator) has enough balance also for the `gas_limit` he may click the `Spawn App` button.
1. Clicking the `Spawn App` button will dispatch the `Spawn App` transaction to the network.


#### `Execute App Transaction (a.k.a Call Method Transaction)`
The steps:

1. Wallet UX user picks the desired app. This user need to have its `Address`.
1. The `execute app` interface is displayed by showing the public API methods of the `App`.
1. User selects the desired API method.
1. User fills-in the method fields.

   Special attention should be given to the `value` field, which is part of the `Transaction Envelope`.
   Amount of `value` will be tranfered to the `App`. (it will be transfered from the `sender`'s balance).
   
   TOD: how to derive the `gas_price` ?
1. The estimated required `gas_limit` is shown to the user.
1. If user (a.k.a `sender`) has enough balance also for the `gas_limit` he may click the `Execute App` button.
1. Clicking the `Execute App` button will dispatch the `Execute App` transaction to the network.


#### `Reading App's Storage`
The steps:

1. Wallet UX user picks the desired app. This user need to have its `Address`.
1. The `App State ABI` is dowloaded and rendered to the user. (off-chain data).
1. Wallet UX invokes a batch call asking for each `App Storage` field. 

TODO: talk about future non-static fields.


### Validation (Mempool)
Each network peer should perform syntactic validation to SVM transactions. 
In case the validation fails, the transaction should be discarded.

### Mining - Which transactions to pick?
The miner will have to decide which transactions are most appealing to him.
Also, we want to have a minimum overlap between miners selections.

The issue with Smart-Contracts is that we only have gas estimation which derives the `gas_limit`.
However, since we only allow a restricted-set of WebAssembly having no loops we can achieve a better estimation.

The total gas estimation will consist of 2 parts:
* Execution estimation 
* Payloyad size - This is a number we can know exactly ahead. 
* Storage size  - We can know-ahead the root hierarchy size (it's specified in the `Template` spec). 

TODO: talk about the algorithm


### Raw Transactions format
We'll need to introduce a transaction type flag to the `Transaction Envelope`

For example:

* type=0  simple transaction with `SVM-ed25519`
* type=1  simple transaction without standard signatures (supported by Hardware Wallets).
* type=2  deploy template. SVM 0.2 should disable that type.

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


### Global State

SVM requires two new account types to be added:

#### `App Template` Account
After deploying a template sucessfully, a new account of type `Template` should be added to the `Global State`.
The `state` of this account should be set to zeros (it's meaningless).
The `balance` of this account should be set to zero.

Sending coins to the `Template` account in any future transaction will lock these for good. 

#### `App` Account
After spawning an App sucessfully, a new account of type `App` should be added to the `Global State`.
The `App`'s initial `state` is returned by the `Spawn Receipt` (see more data under `Receipts` section).
The `balance` of this account should be set with the `value` given by the `Spawn App` transaction sender.

The data for an `App` account will be:

* Address
* Balance 
* App-State 


#### Commiting changes
While executing an `App Transaction`, the app will makes changes to the App's storage and to the balances of accounts.
Upon a successful transaction, SVM will persist the `App storage` changes and re-calculate a new `State`. 
Then, the `Receipt` will include that new `State`. 

Now, the `Global State` should:

* Update the `App` leaf-node with the new App's `State`.
* Apply the dirty coins transfers
* Recalculate the new Merkle-Tree Hashes.


### Receipts
There are 3 types of `Receipt`s: `deploy-template`, `spawn-app` and `exec-app`.
Each `Receipt` should be persisted on-chain in its raw packed form.

Additionally, `SVM` exposes `Receipt helper methods` for extracting each field in isolation.
<br/><br/>
#### `Deploy App-Template` 
If the `is_success` field if `true` it means that the `deploy-transaction` has succedded.
Then, the `template_address` should be extracted for the new `App Template` account creation. (see `Global State` section).

Fields:

* `is_success = true`
* `template_address` 
* `gas_used`

When `is_success` if `false` it means, it means that the `deploy-transaction` has failed.
Now, `go-spacemesh` needs to fee the `sender` with the `gas_limit`.
Both `sender` and `gas_limit` fields are sent as part of the transaction envelope.
<br/><br/>
#### `Spawn App`
When the spawned-app succeeds (`is_success = true`) the returned receipt contains the following:

* `app_address` - The `address` of the spawned-app.
* `init_state`  - The initial `state` of the `App` (after executing the constructor).
* `returns`     - The executed function returned values. Array of `wasm value`. Each value can be `i32` or `i64`.
* `gas_used`    - The amount of gas used.
<br/><br/>
#### `Execute App-Transaction` 
When the executed app-transaction succeeds (`is_success = true`) the returned receipt contains the following:

* `new_state` - The new `state` of the `App`
* `returns`   - The executed function returned values. Array of `wasm value`. Each value can be `i32` or `i64`.
* `gas_used`  - The amount of gas used.


### On-Chain data

* Each transaction should be part of the `Transactions Tree.`
* `App Template` and `App` account will be part of the `Global State`
* `Receipt` should be on-chain too.
* SVM manages the data of each `App` and provides the `App State` to the `Global State`.

### Genesis flow ABI
TBD

### App Storage Read ABI
TBD


#### Other Open Questions

* Signatures Scheme.
* `Receipt` should be part of the `Transactions Mesh` or in other data-structure? 
* `Balance` representation. Can we use a single `i64` or a pair of `i64`? 
* Exact formula for deriving the `Template` and `App` accounts addresses.
* Does the `returns` field of the `Spawn App` and `Exec App` Receipts should be discarded?
  The volume of the this field won't affect the final `gas_used`... 
* We need to figure out what indexes will be created in `go-spacemesh` that will asist the _Transactions Explorer_.

Examples for such indexes.
```
tx_id -> Receipt
layer_id -> [Receipt]
```

* Do we want to have the encoding of each Receipt kind to be the same?
```
(version, receipt_type, is_success, gas_used)
```

### Not be included in SVM 0.2
Here is the list of things that won't be included in SVM 0.2 but must be in the subsequent 0.3 version.

#### Generic Call Method ABI 
Requires more research.

#### Transient Events
We'd like SVM to emit events that will be persisted for an ephemeral amount of time.
By having transient events, we can avoid the feature abuse done on other chains.
This capablity should become very useful for debugging and the transaction Explorer.
The events won't be part of a Receipt. 


[go-spacemesh]: https://github.com/spacemeshos/go-spacemesh
