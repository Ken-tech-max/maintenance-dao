import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MaintenanceDao } from '../target/types/maintenance_dao';

describe('maintenance-dao', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MaintenanceDao as Program<MaintenanceDao>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });

  
});
