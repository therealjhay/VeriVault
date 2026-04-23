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

export class VeriVaultSDK {
  rpcUrl: string;
  contractId: string;

  constructor(rpcUrl: string, contractId: string) {
    this.rpcUrl = rpcUrl;
    this.contractId = contractId;
  }
  
  async connect(wallet: any): Promise<void> {
    // connect wallet
  }
  
  async createNote(attrs: ComplianceAttrs): Promise<Note> {
    return {
      amount: 0,
      commitment: "0x",
      nullifierHash: "0x",
      metadata: "0x"
    };
  }
  
  async deposit(note: Note): Promise<any> {
    return {};
  }
  
  async withdraw(note: Note, recipient: string): Promise<any> {
    return {};
  }
  
  onMerkleUpdate(callback: (root: string) => void): () => void {
    return () => {};
  }
}
