import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SuperPay } from '../target/types/super_pay';

describe('super-pay', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SuperPay as Program<SuperPay>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
