## Integration with `go-spacemesh` plan
This document is intended to serve as high-level plan for integration of SVM with the [`go-spacemesh`][go-spacemesh].

There are two main purposes for this doc:
* Make sure all the spacemesh team members are aligned with the high-level plan and terminology.
* Form a base from which GitHub issues will be created.

Note: since SVM is a standalone project this document may be a good reference for any other future Blockchain project willing to integrate SVM.

### Terminology

* `App Template`
We name a `Smart Contract`'s code + metadata as `App Template`.
We can think of a `Template` as the matching of an Object-Oriented `class`. 

* `App`
Given an `App Template` we can spawn `App`s out of it.
All spawned `App`s out-of the same `Template` share the same code but have an isloated inner state.
We can think of an `App` as the matching of an Object-Oriented `class instance`. 
The motivation for having both `App Template` and `App` are to encourage code reuse and saving on-chain storage.

* `App-Transaction` 
Given a spawned `App` we'd like to execute `App Transaction` on it.



### High-level flows
* `Deploy App Template`

```
Wallet UX -- gRPC --> go-spacemesh ----> go-svm ----> SVM
                      go-spacemesh <---- go-svm <---- SVM
							 --> dispatch (p2p)
```

Since `go-spacemesh` v0.2 will come with a single built-in template, named `MultiSig Wallet`.
Therefore, the `deploy-template` functionality using the `p2p` should be disabled.

See `Genesis flow` for how to deploy the `MultiSig Wallet`.

* `Spawn App`


* `Execute App Transaction`


### Validation (Mempool)
* `Template validation`

* `Template App`

* `App Transaction`

### Mining
* Which transactions pick?

 
### Raw Transaction format

### p2p


### Global State

* `App Template` Account

* `App` Account


### Receipts

#### `Deploy App-Template` 

Fields:
* `is_success`
* `template_address` 
* `gas_used`
* `error` 

#### `Spawn App`
Fields:
* `is_success`
* `app_address`
* `init_state`
* `returns`
* `gas_used`
* `error`

#### `Execute App-Transaction` 
Fields:
* `is_success`
* `new_state`
* `returns`
* `gas_used`
* `error`


### On-Chain data


### Wallet UX / Wallet CLI API

* App State ABI

`Wallet UX` -- gRPC --> `go-spacemesh` -- `go-svm` --> `SVM`  


[go-spacemesh]: https://github.com/spacemeshos/go-spacemesh
