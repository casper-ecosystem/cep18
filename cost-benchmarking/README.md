# NCTL Cost Benchmarking Scripts

These scripts deploy and operate a basic CEP-18 contract to record the gas costs of basic operations. Output is written to `cep18-cost-benchmarking-output` in the repository's root directory.

## Prerequisites
- a running NCTL network with casper-node version 1.4.4 or later
- user 1 with sufficient tokens (usually the case!)
- users 1-3 available
- user 1 always acts as the installer

## Scripts

The first script makes sure that the contract is compiled, installs it, and records the cost of installation:

`example/cost-benchmarking/prepare.sh`

The second script exercises all entry points with different amounts and records the costs.

`example/cost-benchmarking/run-benchmarks.sh`

## Typical Operation

Invoke `. example/prepare.sh` from the repository's root, then invoke `. example/run-benchmarks.sh` if needed.