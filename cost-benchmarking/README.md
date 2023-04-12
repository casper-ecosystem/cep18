NCTL cost benchmarking scripts
======================

These scripts deploy and operate the basic ERC20 contract to record gas costs of basic operations. Output is written to `erc20-cost-benchmarking-output` in the repo root directory.

Assumptions
-----------
- running NCTL network with casper-node version 1.4.4 or later
- user 1 with sufficient tokens (normally the case!)
- users 1-3 available
- user 1 always acts as the installer

Scripts
-------------------

`example/cost-benchmarking/prepare.sh`

Makes sure that the contract is compiled, installs it and records the cost of installation.

`example/cost-benchmarking/run-benchmarks.sh`

Exercises all entry points with different amounts specified and records the costs.

Typical operation
-------------------

Invoke `. example/prepare.sh` from repo root, then invoke `. example/run-benchmarks.sh` if needed.