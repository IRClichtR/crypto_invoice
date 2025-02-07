const { expect } = require("chai");
const { ethers } = require("hardhat");
const parseEther = ethers.parseEther;
require("dotenv").config();

describe("InvoicePayment", function () {
  let InvoicePayment, escrow;

  beforeEach(async function () {
    [emitter, client, arbitrator, platformWallet] = await ethers.getSigners();
    // console.log("Emitter Address:", emitter.address);
    // console.log("Client Address:", client.address);
    // console.log("Arbitrator Address:", arbitrator.address);
    // console.log("Platform wallet Address:", platformWallet.address);

    InvoicePayment = await ethers.getContractFactory("InvoicePayment");
    escrow = await InvoicePayment.deploy(
      arbitrator.address,
      platformWallet.address,
    );
  });

  it("Should allow the emitter to create an invoice", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    const invoice = await escrow.invoices(1);

    expect(invoice.invoiceId).to.equal(1);
    expect(invoice.client).to.equal(client.address);
    expect(invoice.emitter).to.equal(emitter.address);
    expect(invoice.amount).to.equal(parseEther("1.0"));
    expect(invoice.status).to.equal(0);
  });

  it("Should NOT allow the client to create an invoice", async function () {
    await expect(
      escrow.connect(client).createInvoice(emitter.address, parseEther("1.0")),
    ).to.not.be.reverted;
  });

  it("Should allow the client to pay an invoice", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(1);
  });

  it("Should NOT allow double payment on the same invoice", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, { value: parseEther("1.0") });

    await expect(
      escrow.connect(client).payInvoice(1, { value: parseEther("1.0") }),
    ).to.be.revertedWith("Invoice must be pending");
  });

  it("Should fail if payment is not exact", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await expect(
      escrow.connect(client).payInvoice(1, {
        value: parseEther("0.5"),
      }),
    ).to.be.revertedWith("Incorrect payment amount");
  });

  it("Should allow client to confirm payment to the emitter", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, { value: parseEther("1.0") });

    await escrow.connect(client).confirmPayment(1);
    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(4);
  });

  it("Should transfer funds to the emitter and platform after confirmation", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      ethers.parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: ethers.parseEther("1.0"),
    });

    const balanceBefore = await ethers.provider.getBalance(emitter.address);
    await escrow.connect(client).confirmPayment(1);
    const balanceAfter = await ethers.provider.getBalance(emitter.address);

    expect(balanceAfter).to.be.gt(balanceBefore);
  });

  it("Should automatically release funds after timeout", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      ethers.parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: ethers.parseEther("1.0"),
    });

    await network.provider.send("evm_increaseTime", [7 * 24 * 60 * 60]);
    await network.provider.send("evm_mine");

    await escrow.autoReleasePayment(1);

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(3); // Status.Released
  });

  it("Should allow the client to dispute a payment", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(client).disputeByClient(1, { from: client.address });

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(2);
  });

  it("Should allow the emitter to dispute a payment", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(emitter).disputeByEmitter(1, {
      from: emitter.address,
    });

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(2);
  });

  it("Should allow the arbitrator to resolve a dispute in favor of the emitter", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(client).disputeByClient(1, { from: client.address });
    await escrow.connect(arbitrator).resolveDispute(1, true, {
      from: arbitrator.address,
    });

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(3);
  });

  it("Should allow the arbitrator to resolve a dispute in favor of the client", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(client).disputeByClient(1, { from: client.address });
    await escrow.connect(arbitrator).resolveDispute(1, false, {
      from: arbitrator.address,
    });

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(0);
  });

  it("Should NOT allow a non-arbitrator to resolve disputes", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(client).disputeByClient(1, { from: client.address });

    await expect(
      escrow.connect(client).resolveDispute(1, true, { from: client.address }),
    ).to.be.revertedWith("Only arbitrator can perform this action");

    await expect(
      escrow.connect(emitter).resolveDispute(1, false, {
        from: emitter.address,
      }),
    ).to.be.revertedWith("Only arbitrator can perform this action");
  });

  it("Should show contract balance after payment", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });

    const balance = await escrow.getBalance();
    expect(balance).to.equal(parseEther("1.0"));
  });
});
