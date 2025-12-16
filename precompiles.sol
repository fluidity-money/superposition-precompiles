// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

library muldiv {
    function mul_div(uint256 x, uint256 y, uint256 z) external view returns (
        bool wasOverflow,
        uint256 answer
    ) {
        (bool rc, bytes memory rd) = 0x6c483D05266CDa72cFE72643A79Ad531d9B52cd5.staticcall(abi.encode(x, y, z));
        assert(rc);
        return abi.decode(rd, (bool, uint256));
    }
}

library edphverify {
    function edphverify(
        bytes32 digestA,
        bytes32 digestB,
        bytes32 publicKey,
        bytes32 sigA,
        bytes32 sigB
    ) external view {
        (bool rc,) = 0xC3E443bE2Cfa4F41a5F5E4978D012847d355b419.staticcall(abi.encode(
            digestA,
            digestB,
            publicKey,
            sigA,
            sigB
        ));
        require(rc);
    }
}
