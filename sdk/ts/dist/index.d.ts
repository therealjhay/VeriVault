export interface ComplianceAttrs {
    kycLevel: number;
    geoRegion: string;
}
export interface Note {
    amount: number;
    commitment: string;
    nullifierHash: string;
    metadata: string;
}
export declare class VeriVaultSDK {
    rpcUrl: string;
    contractId: string;
    constructor(rpcUrl: string, contractId: string);
    connect(wallet: any): Promise<void>;
    createNote(attrs: ComplianceAttrs): Promise<Note>;
    deposit(note: Note): Promise<any>;
    withdraw(note: Note, recipient: string): Promise<any>;
    onMerkleUpdate(callback: (root: string) => void): () => void;
}
