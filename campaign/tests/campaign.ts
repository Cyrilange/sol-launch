const { expect } = require("chai");
const { AnchorProvider, web3 } = require("@project-serum/anchor");
const {
  createCampaign,
  contributeToCampaign,
  createRequest,
  approveRequest,
} = require("../programs/campaign/src");

const provider = AnchorProvider.local();
const connection = provider.connection;
const wallet = provider.wallet;

let campaignAccount;
let campaignAccountData;

beforeAll(async () => {
  await provider.connection.requestAirdrop(wallet.publicKey, 100000000000);
});

test("create campaign", async () => {
  const { account, data } = await createCampaign(connection, wallet.payer);
  campaignAccount = account;
  campaignAccountData = data;

  expect(campaignAccountData.campaign.manager).toEqual(
    wallet.payer.publicKey.toBase58()
  );
  expect(campaignAccountData.campaign.approvers_count).toEqual(0);
  expect(campaignAccountData.campaign.requests.length).toEqual(0);
});

test("contribute to campaign", async () => {
  const contributionAmount = 1;
  const { account, data } = await contributeToCampaign(
    connection,
    wallet.payer,
    campaignAccount,
    contributionAmount
  );

  expect(data.campaign.approvers_count).toEqual(1);
  expect(data.campaign.approvers[0]).toEqual(wallet.payer.publicKey.toBase58());
});

test("create request", async () => {
  const description = "Buy a new computer";
  const value = 5;
  const recipient = new web3.PublicKey("recipient_address");
  const { account, data } = await createRequest(
    connection,
    wallet.payer,
    campaignAccount,
    description,
    value,
    recipient
  );

  expect(data.campaign.requests.length).toEqual(1);
  expect(data.campaign.requests[0].description).toEqual(description);
  expect(data.campaign.requests[0].value).toEqual(value);
  expect(data.campaign.requests[0].recipient).toEqual(recipient.toBase58());
});

test("approve request", async () => {
  const requestIndex = 0;
  const { account, data } = await approveRequest(
    connection,
    wallet.payer,
    campaignAccount,
    requestIndex
  );

  expect(data.campaign.requests[0].approvals[0]).toEqual(true);
  expect(data.campaign.requests[0].approval_count).toEqual(1);
});
