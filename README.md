# Nope

*Nope* is a hyper-secure scripting language and RISC virtual machine for performant and risk-free on-chain execution.

- [nopec](./nopec/README.md), the canonical *NopeLang* compiler
- [*Nopelang*](./lang/README.md), the *NopeLang* specification

All on-chain execution environments must choose a balance between complexity and security. While some environments such as CosmWasm or the EVM enable a high degree of expressiveness for complex applications, this complexity also creates the potential for many security vulnerabilities. Other environments such as the MoveVM address this by enforcing certain invariants and restrictions at compile time, however certain runtime checks must still be done.

*Nope* takes this a step further: all features and complexities in both the language and bytecode of existing on-chain execution environments have been carefully evaluated, and any that could open the door to a potential security vulnerability were not included in the *NopeLang* or *NopeVM* specifications.

- *Cross-contract calls*: While calling into arbitrary contracts can be very powerful, it is also a very large security vulnerability due to potential reentrancy attacks, or even calling the wrong contract due to malicious or incorrectly verified input. The complexity and security considerations of this functionality far outweighs the benefits.

- *Multiple entrypoints*: Most execution environments support contracts with multiple entrypoints. While this can be useful for certain usecases, it also introduces a significant security concern as every additional entrypoint is an additional attack vector. Rather than force the burden of manually verifying all invariants for every entrypoint, *NopeLang* instead only supports a single entrypoint per script.

- *State*: Incorrectly managed state is a source of many vulnerabilities. Similar to the reasoning above, *NopeLang* lifts this burden from the developer and does not support persistent state in scripts between script executions.

- *Transaction input*: Malicious input is the root cause of the majority of on-chain exploits. As such, entrypoints in *NopeLang* do not support receiving external input, removing all possible security vulnerabilities caused by external actors.

Additionally, the instruction set of the *NopeVM* has been designed in such a way so as to not contain any opcodes which could cause a security vulnerability:

- *Arithmetic*: It is unreasonable to expect a developer to fully grasp the complexities of many potentially-correlated arithmetic operations, and therefore it is also unreasonable to expect that same developer to correctly reason about all possible outcomes of said operations. Additionally, numeric over/underflows are a common attack vector if not handled correctly. To mitigate this, the *NopeVM* does not support any arithmetic operations.

- *Branching*: Branching introduces significant complexity to any execution environment. Properly tracking all possible leaves of a program containing branches introduces significant cognative overhead, potentially creating attack vectors if even one of the aforementioned leaves are missed or tracked incorrectly. To prevent this, *NopeVM* does not support any form of branching or boolean operations.

- *Precompiles*: While complex precompiles can be useful in certain usecases or specific environments, the *NopeVM* aims to be general-purpose, and therefore does not support specific precompiles.

- *Memory*: Out-of-bounds memory accesses are a common problem both on- and off-chain. While certain languages have their own mitigation strategies for this class of problems (dependent typing, fixed-size collections, or only supporting checked memory reads, to name a few), *NopeVM* takes a somewhat unique approach by not having any read/write memory at all. This ensures, at the lowest possible level, that no invalid memory access operations are run.

Taking all of this into consideration results in the following opcodes:

| opcode | format | description  | gas cost |
| ------ | ------ | ------------ | -------- |
| `NOOP` | `0x00` | no operation | `1`      |
