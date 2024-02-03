const web3 = require("@solana/web3.js");
const { SystemProgram } = web3;
const anchor = require("@project-serum/anchor");
const idl = require("./idl.json");

const provider = anchor.Provider.env();
const program = new anchor.Program(idl, "campaign_factory", provider);

const createCampaignBtn = document.getElementById("create-campaign-btn");

createCampaignBtn.addEventListener("click", async () => {
  const minimumContribution = 100; // in lamports
  const [campaignAddress, _] = await web3.PublicKey.findProgramAddress(
    [Buffer.from("campaign")],
    program.programId
  );

  try {
    await program.methods
      .createCampaign(minimumContribution)
      .accounts({
        campaignAccount: campaignAddress,
        manager: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .sign({
        wallet: provider.wallet,
      })
      .rpc();

    console.log("Campaign created successfully!");
  } catch (error) {
    console.error("Error creating campaign:", error);
  }
});
