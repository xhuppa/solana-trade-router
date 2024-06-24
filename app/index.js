import { Connection, PublicKey, SystemProgram, Transaction, Keypair, clusterApiUrl } from '@solana/web3.js';
import fs from 'fs';
import path from 'path';
import dotenv from 'dotenv';

dotenv.config();

const getKeypair = () => {
  const filePath = path.join(process.cwd(), 'keypair.json');
  const keypairData = JSON.parse(fs.readFileSync(filePath, 'utf-8'));

  const secretKey = Uint8Array.from(Buffer.from(keypairData.privateKey, 'base64'));
  return Keypair.fromSecretKey(secretKey);
};

const requestAirdropWithRetry = async (connection, publicKey, retries = 3, delay = 2000) => {
  for (let i = 0; i < retries; i++) {
    try {
      const airdropSignature = await connection.requestAirdrop(publicKey, 2 * 1e9);
      await connection.confirmTransaction(airdropSignature);
      return;
    } catch (error) {
      console.error(`Airdrop failed: ${error.message}. Retrying (${i + 1}/${retries})...`);
      if (i < retries - 1) {
        await new Promise(resolve => setTimeout(resolve, delay));
      } else {
        throw new Error('Airdrop failed after maximum retries');
      }
    }
  }
};

const interactWithProgram = async () => {
  const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

  const payer = getKeypair();

  const targetPublicKey = new PublicKey(process.env.TARGET_PUBLIC_KEY || 'YourTargetPublicKeyHere');

  await requestAirdropWithRetry(connection, payer.publicKey);

  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: targetPublicKey,
      lamports: 1e6,
    })
  );

  const signature = await connection.sendTransaction(transaction, [payer]);
  await connection.confirmTransaction(signature);

  console.log(`Transaction confirmed with signature: ${signature}`);
};

interactWithProgram().catch(console.error);
