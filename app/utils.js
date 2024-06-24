import { Connection, PublicKey, clusterApiUrl, Keypair } from '@solana/web3.js';
import fs from 'fs';
import path from 'path';

export const getConnection = () => {
    return new Connection(process.env.SOLANA_RPC_URL, 'confirmed');
};

export const getProgramId = () => {
    return new PublicKey(process.env.PROGRAM_ID);
};

export const generateAndSaveKeypair = () => {
  const keypair = Keypair.generate();

  const publicKey = keypair.publicKey.toBase58();
  const privateKey = Buffer.from(keypair.secretKey).toString('base64');

  const keypairData = {
    publicKey,
    privateKey,
  };

  const filePath = path.join(process.cwd(), 'keypair.json');
  fs.writeFileSync(filePath, JSON.stringify(keypairData, null, 2));

  console.log(`New Public Key: ${publicKey}`);
  console.log(`Keypair saved to ${filePath}`);
};

generateAndSaveKeypair();
