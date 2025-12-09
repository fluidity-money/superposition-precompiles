
# superposition-precompiles

## Ed25519 verifier

Deployed by address `0x37f8c060dbeb1786deb6a88f387200642122be76` to
`0xc3e443be2cfa4f41a5f5e4978d012847d355b419` on Superposition and Arbitrum One.

### Why not precomputed tables?

We did some comparison, and found:

```
precomputed	Ed25519COMPARISON:test_fuzzEd25519(bytes32,bytes) (runs: 257, μ: 93467, ~: 93511)
no precomputed	Ed25519COMPARISON:test_fuzzEd25519(bytes32,bytes) (runs: 257, μ: 92715, ~: 92703)
```

## Building

	make

## Testing

Online testing is tested with bobcat-sdk.

	./tests.sh
