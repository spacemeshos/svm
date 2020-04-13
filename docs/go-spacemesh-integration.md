## Integration with `go-spacemesh` plan
This document is intended to serve as high-level plan for integration of SVM with [`go-spacemesh`][go-spacemesh].

There are two main purposes for this doc:
* Make sure that spacemesh team members are aligned with the high-level plan and terminology.
* Form a basis from which GitHub issues will be created.

Note: since SVM is a standalone project this document may be a good reference for any other future Blockchain project willing to integrate SVM.

### Terminology

* `Transaction Envelope`
<br/>
This term refers to any transaction data besides SVM specific data.
It will be mentioned usually in the context of transaction fields such as: `sender`, `value`, `gas_limit`, `gas_price` and `nonce`.

* `Host Context`
<br/>
This term refers to the context of the host. Meaning, the data of `Transaction Envelope` plus extra data.
It will contain fields such as: `block_id`, `layer_id`..

Executed SVM transactions will have access to the Host Context.
The data-structure used for the Host Context will be a map between an i32 integer index to raw byte array.

``` 
{
	# `0` denotes `nonce` in this example.
	0 => [0x10, 0x20],
	
	# `1' denotes `layer_id` in this example. 
	1 => [0x1A, 0x5C, 0x2D]
}
```

* `App Template`
<br/>
We name a `Smart Contract`'s code + metadata as a `App Template`.
<br/>
We can think of a `Template` as the equivalent of a `class` in an Object-Oriented programing paradigm.
<br/>
Each `Template` will have an account under the `Global State` and its own `Addres`. (see more under the `Global State` section).

* `App`
<br/>
Given an `App Template` we can spawn `App`s out of it.
All spawned `App`s out-of the same origin `Template` share the same code but have an isloated inner state.
<br/>
We can think of an `App` as the equivalent of a `class instance` (a.k.a `object`) in an Object-Oriented programing paradigm.
<br/>
The motivation for having both `App Template` and `App` are to encourage code reuse and saving of on-chain storage.
<br/>
Each `App` will have an account under the `Global State` and its own `Addres`. (see more under the `Global State` section).

* `App-Transaction` 
<br/>
Given a spawned `App` we'd like to execute `App Transaction`s on it.
<br/>
We can think of executing an `App Transaction` as the equivalent of a invoking an `object method` in an Object-Oriented programing paradigm.
<br/>
Executing `App Transaction` are the way to apply changes and transaction the state of an `App`. 


### High-level flows
<br/>
#### `Deploy App Template`

The `go-spacemesh` v0.2 will contain only a single built-in template, named `MultiSig Wallet`.
Therefore, the `deploy-template` functionality using the `p2p` should be disabled.

See `Genesis flow` for how to deploy the pre-built `MultiSig Wallet`.


#### `Spawn App`
<br/>
The `go-spacemesh` v0.2 will support only apps of the `MultiSig Wallet` template.
Part of the apps will be spawned as part of the `Genesis flow` and the rest apps will be spawned via the `Wallet UX` client. 
<br/>

The steps:
1. Wallet UX picks the required template. For `go-spacemesh` v0.2 the template will always be the `MultiSig Wallet`.
1. The `spawn app` interface is displayed with constructor input fields derived from the `App Template ABI`.

   Special attention should be given to the `value` field, which is part of the `Transaction Envelope`.
   The balance of the `spawned-app` will be set to that `value`. (it will be transfered from the app's creator balance).
   
   TOD: how to derive the `gas_price` ?
   
   Both `value` and `gas_price` are part of any app spawning.

1. User fills-in the constructor fields.
1. The estimated required `gas_limit` is shown to the user.
1. If user (app creator) has enough balance also for the `gas_limit` he may click the `Spawn App` button.
1. Clicking the `Spawn App` button will dispatch the `Spawn App` transaction to the network.


* `Execute App Transaction`
TBD


* `Read App State`
TBD


### Validation (Mempool)
* `Template validation`
TBD

* `Template App`
TBD

* `App Transaction`
TBD


### Mining
* Which transactions pick?
TBD

 
### Raw Transaction format
TBD


### p2p
TBD


### Global State

#### `App Template` Account
After deploying a template sucessfully, a new account of type `Template` should be added to the `Global State`.
The `state` of this account should be set to zeros (it's meaningless).
The `balance` of this account should be set to zero.

Sending coins to the `Template` account in any future transaction will lock these for good. 


#### `App` Account
After spawning an App sucessfully, a new account of type `App` should be added to the `Global State`.
The `App`'s initial `state` is returned by the `Spawn Receipt` (see more data under `Receipts` section).
The `balance` of this account should be set with the `value` given by the `Spawn App` transaction sender.


### Receipts

There are 3 types of `Recepit`s (for `deploy-template`, `spawn-app` and `exec-app`).
Each `Receipt` should be persisted on-chain in its raw packed form.

Additionally, `SVM` exposes `Receipt helper methods` for extracting each field in isolation.

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


#### `Spawn App`
When the spawned-app succeeds (`is_success = true`) the returned receipt contains the following:
* `app_address` - The `address` of the spawned-app.
* `init_state`  - The initial `state` of the `App` (after executing the constructor).
* `returns`     - The executed function returned values. Array of `wasm value`. Each value can be `i32` or `i64`.
* `gas_used`    - The amount of gas used.

#### `Execute App-Transaction` 
When the executed app-transaction succeeds (`is_success = true`) the returned receipt contains the following:
* `new_state` - The new `state` of the `App`
* `returns`   - The executed function returned values. Array of `wasm value`. Each value can be `i32` or `i64`.
* `gas_used`  - The amount of gas used.


### On-Chain data
Each transaction should be part of the `Transactions Tree.`


### Wallet UX / Wallet CLI API

* App State ABI

```
Wallet UX -- gRPC --> go-spacemesh ----> go-svm ----> SVM 
```

[go-spacemesh]: https://github.com/spacemeshos/go-spacemesh
