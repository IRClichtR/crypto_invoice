pragma solidity 0.8.28;

import "@openzeppelin/contracts/access/Ownable.sol";

contract InvoicePayment is Ownable {
  enum Status { Pending, Paid, Disputed, Released }

  struct Invoice {
    uint256 invoiceId;
    address payable client;
    address payable emitter;
    uint256 amount;
    Status status;
  }

  mapping(uint256 => Invoice)public invoices;
  uint256 public invoiceCount;
  address public arbitrator;
  address payable public platformWallet;
  uint256 public platformFeePercent = 2;

  event InvoiceCreated(uint256 invoiceId, address client, address emitter, uint256 amount);
  event PaymentMade(uint256 indexed invoiceId, uint256 amount, uint256 fee);
  event PaymentReleased(uint256 indexed invoiceId, uint256 amount);
  event PaymentDisputed(uint256 indexed invoiceId, address disputor);
  event ArbitratorChanged(address newArbitrator);
  event DisputeResolved(uint256 indexed invoiceId, bool releasedToEmitter);

  modifier onlyClient(uint256 _invoiceId) {
    require(msg.sender == invoices[_invoiceId].client, "Only client can perform this action");
    _;
  }

  modifier onlyEmitter(uint256 _invoiceId) {
    require(msg.sender == invoices[_invoiceId].emitter, "Only emitter can perform this action");
    _;
  }

  modifier onlyArbitrator() {
    require(msg.sender == arbitrator, "Only client can perform this action");
    _;
  }

  constructor(address _arbitrator, address payable _platformWallet) Ownable(msg.sender) {
    arbitrator = _arbitrator;
    platformWallet = _platformWallet;
  }

  function setArbitrator(address _newArbitrator) public onlyOwner {
    arbitrator = _newArbitrator;
    emit ArbitratorChanged(_newArbitrator);
  }

  function setPlatformFeePercent(uint256 _newFee) public onlyOwner {
    require(_newFee <= 10, "Fee cannot exceed 0%");
    platformFeePercent = _newFee;
  }

  function createInvoice(address payable _client, uint256 _amount) public {
    invoiceCount++;

    invoices[invoiceCount] = Invoice(
      invoiceCount,
      _client,
      payable(msg.sender), //emitter is calling the function then msg.sender refersto it
      _amount,
      Status.Pending
    );

    emit InvoiceCreated(invoiceCount, _client, msg.sender, _amount);
  }

  function payInvoice(uint256 _invoiceId) public payable onlyClient(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Pending, "Invoice must be pending");
    require(msg.value == invoice.amount, "Incorrect payment amount");

    uint256 fee = (msg.value * platformFeePercent) / 100;
    uint256 amountToEmitter = msg.value - fee;

    platformWallet.transfer(fee);
    invoice.status = Status.Paid;

    emit PaymentMade(_invoiceId, amountToEmitter, fee);
  }

  function disputeInvoiceByClient(uint256 _invoiceId) public onlyClient(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Paid, "Payment must be in progress");

    invoice.status = Status.Disputed;
    emit PaymentDisputed(_invoiceId, msg.sender);
  }

  function disputeInvoiceByEmitter(uint256 _invoiceId) public onlyEmitter(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Paid, "Payment must be in progress");

    invoice.status = Status.Disputed;
    emit PaymentDisputed(_invoiceId, msg.sender);
  }

  function releasePayment(uint256 _invoiceId) public onlyClient(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Paid, "Payment must be completed first");

    invoice.status = Status.Released;
    invoice.emitter.transfer(invoice.amount);
    emit PaymentReleased(_invoiceId, invoice.amount);
  }

  function resolveDispute(uint256 _invoiceId, bool releaseToEmitter) public onlyArbitrator {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Disputed, "Invoice is not in dispute");

    if (releaseToEmitter) {
      invoice.emitter.transfer(invoice.amount);
      invoice.status = Status.Released;
    } else {
      invoice.client.transfer(invoice.amount);
      invoice.status = Status.Pending;
    }

    emit DisputeResolved(_invoiceId, releaseToEmitter);
  }
}
