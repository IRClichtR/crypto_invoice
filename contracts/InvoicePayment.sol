pragma solidity 0.8.28;

import "@openzeppelin/contracts/access/Ownable.sol";

contract InvoicePayment is Ownable {
  enum Status { Pending, Paid, Disputed, Released, Validated }

  struct Invoice {
    uint256 invoiceId;
    address payable client;
    address payable emitter;
    uint256 amount;
    uint256 paymentTimeStamp;
    Status status;
  }

  mapping(uint256 => Invoice)public invoices;
  uint256 public invoiceCount;
  address public arbitrator;
  address payable public platformWallet;
  uint256 public platformFeePercent = 2;
  uint256 public paymentTimeout = 7 days;

  event InvoiceCreated(uint256 invoiceId, address client, address emitter, uint256 amount);
  event PaymentMade(uint256 indexed invoiceId, uint256 amount);
  event PaymentConfirmed(uint256 indexed invoiceId);
  event PaymentReleased(uint256 indexed invoiceId, uint256 amount, uint256 fee);
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
    require(msg.sender == arbitrator, "Only arbitrator can perform this action");
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

  function setPaymentTimeout(uint256 _newTimeout) public onlyOwner {
    require(_newTimeout >= 1 days, "Timeout must be at least one day");
    paymentTimeout = _newTimeout;
  }

function createInvoice(address payable _client, uint256 _amount) public {
    invoiceCount++;

    invoices[invoiceCount] = Invoice({
        invoiceId: invoiceCount,
        client: _client,
        emitter: payable(msg.sender),
        amount: _amount,
        paymentTimeStamp: 0,
        status: Status.Pending
    });

    emit InvoiceCreated(invoiceCount, _client, msg.sender, _amount);
}

  function getBalance() public view returns (uint256) {
    return address(this).balance;
  }

  function payInvoice(uint256 _invoiceId) public payable onlyClient(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Pending, "Invoice must be pending");
    require(msg.value == invoice.amount, "Incorrect payment amount");
    require(msg.value <= address(msg.sender).balance, "Insufficient balance to pay Invoice");

    invoice.paymentTimeStamp = block.timestamp;
    invoice.status = Status.Paid;

    emit PaymentMade(_invoiceId, msg.value);
  }

  function confirmPayment(uint256 _invoiceId) public onlyClient(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Paid, 'invoice must be paid first');
    
    uint256 fee = (invoice.amount * platformFeePercent) / 100;
    uint256 amountToEmitter = invoice.amount - fee;

    platformWallet.transfer(fee);
    invoice.emitter.transfer(amountToEmitter);

    invoice.status = Status.Validated;
    emit PaymentConfirmed(_invoiceId);
    emit PaymentReleased(_invoiceId, amountToEmitter, fee);
  }

  function autoReleasePayment(uint256 _invoiceId) public {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Paid, 'invoice must be paid first');
    require(block.timestamp >= invoice.paymentTimeStamp + paymentTimeout, "Timeout not reached");

    uint256 fee = (invoice.amount * platformFeePercent) / 100;
    uint256 amountToEmitter = invoice.amount - fee;

    platformWallet.transfer(fee);
    invoice.emitter.transfer(amountToEmitter);

    invoice.status = Status.Released;
    emit PaymentReleased(_invoiceId, amountToEmitter, fee);
  }

  function disputeByClient(uint256 _invoiceId) public onlyClient(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Paid, "Payment must be in progress");

    invoice.status = Status.Disputed;
    emit PaymentDisputed(_invoiceId, msg.sender);
  }

  function disputeByEmitter(uint256 _invoiceId) public onlyEmitter(_invoiceId) {
    Invoice storage invoice = invoices[_invoiceId];
    require(invoice.status == Status.Paid, "Payment must be in progress");

    invoice.status = Status.Disputed;
    emit PaymentDisputed(_invoiceId, msg.sender);
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
