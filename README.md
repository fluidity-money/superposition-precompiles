
# superposition-precompiles

## Ed25519ph verifier

Deployed by address `0x37f8c060dbeb1786deb6a88f387200642122be76` to
`0xc3e443be2cfa4f41a5f5e4978d012847d355b419` on Superposition and Arbitrum One.

### Why not precomputed tables?

We did some comparison, and found:

```
precomputed	Ed25519COMPARISON:test_fuzzEd25519(bytes32,bytes) (runs: 257, μ: 93467, ~: 93511)
no precomputed	Ed25519COMPARISON:test_fuzzEd25519(bytes32,bytes) (runs: 257, μ: 92715, ~: 92703)
```

## Muldiv using Ruint

Deployed by address `0xed888d33db1bd4f076ba86638f80c14aada9c2fa` to
`0x6c483d05266cda72cfe72643a79ad531d9b52cd5` on Superposition and Arbitrum One.

## Building

	make

## Testing

Online testing is tested with bobcat-sdk.

	./tests.sh
