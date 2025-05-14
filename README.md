# Schnorr Sigma Protocol (Rust Implementation)

This is a modular Rust implementation of the Schnorr Sigma Protocol using the Fiat-Shamir heuristic.

## Overview

Prover proves knowledge of `x` such that `y = g^x mod p` without revealing `x`.

### Protocol Steps
1. Prover computes `a = g^r mod p`
2. Prover derives challenge `c = H(a)`
3. Prover computes response `z = r + c * x`
4. Verifier checks if `g^z == a * y^c`

## Usage

### Run the project:

cargo run

You should see output like:

Verification: true

or

Verification: false