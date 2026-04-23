"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.VeriVaultSDK = void 0;
class VeriVaultSDK {
    constructor(rpcUrl, contractId) {
        this.rpcUrl = rpcUrl;
        this.contractId = contractId;
    }
    async connect(wallet) {
        // connect wallet
    }
    async createNote(attrs) {
        return {
            amount: 0,
            commitment: "0x",
            nullifierHash: "0x",
            metadata: "0x"
        };
    }
    async deposit(note) {
        return {};
    }
    async withdraw(note, recipient) {
        return {};
    }
    onMerkleUpdate(callback) {
        return () => { };
    }
}
exports.VeriVaultSDK = VeriVaultSDK;
