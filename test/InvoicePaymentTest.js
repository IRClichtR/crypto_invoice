const { expect } = require("chai");
const { ethers } = require("hardhat");
const parseEther = ethers.parseEther;

describe("InvoicePayment", function () {
  let InvoicePayment,
    escrow,
    owner,
    client,
    emitter,
    arbitrator,
    platformWallet;

  beforeEach(async function () {
    [owner, client, emitter, arbitrator, platformWallet] = await ethers
      .getSigners();
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
      escrow.connect(client).createInvoice(
        emitter.address,
        parseEther("1.0"),
      ),
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

  it("Should allow the client to release payment to the emitter", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(client).releasePayment(1);

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(3);
  });

  it("Should NOT allow a client to release payment before paying", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await expect(
      escrow.connect(client).releasePayment(1),
    ).to.be.revertedWith("Payment must be completed first");
  });

  it("Should allow the client to dispute a payment", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(client).disputeInvoiceByClient(1);

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
    await escrow.connect(emitter).disputeInvoiceByEmitter(1);

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
    await escrow.connect(client).disputeInvoiceByClient(1);
    await escrow.connect(arbitrator).resolveDispute(1, true);

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
    await escrow.connect(client).disputeInvoiceByClient(1);
    await escrow.connect(arbitrator).resolveDispute(1, false);

    const invoice = await escrow.invoices(1);
    expect(invoice.status).to.equal(0); // Status.Pending (client refunded)
  });

  it("Should NOT allow a non-arbitrator to resolve disputes", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    await escrow.connect(client).payInvoice(1, {
      value: parseEther("1.0"),
    });
    await escrow.connect(client).disputeInvoiceByClient(1);

    await expect(
      escrow.connect(client).resolveDispute(1, true),
    ).to.be.revertedWith("Only arbitrator can perform this action");

    await expect(
      escrow.connect(emitter).resolveDispute(1, false),
    ).to.be.revertedWith("Only arbitrator can perform this action");
  });

  it("Should show contract balance after payment", async function () {
    await escrow.connect(emitter).createInvoice(
      client.address,
      parseEther("1.0"),
    );
    escrow.connect(client).payInvoice(1, { value: parseEther("1.0") });

    const balance = await escrow.getBalance();
    expect(balance).to.equal(parseEther("1.0"));
  });
});
