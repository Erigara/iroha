import os
import binascii
from iroha import IrohaCrypto, Iroha, IrohaGrpc, primitive_pb2
from functools import wraps
import integration_helpers

IROHA_HOST_ADDR = os.getenv("IROHA_HOST_ADDR", "127.0.0.1")
IROHA_PORT = os.getenv("IROHA_PORT", "50051")

net = IrohaGrpc(f"{IROHA_HOST_ADDR}:{IROHA_PORT}")

ADMIN_ACCOUNT_ID = os.getenv("ADMIN_ACCOUNT_ID", "admin@test")
ADMIN_PRIVATE_KEY = os.getenv(
    "ADMIN_PRIVATE_KEY",
    "f101537e319568c765b2cc89698325604991dca57b9716b58016b253506cab70",
)
iroha_admin = Iroha(ADMIN_ACCOUNT_ID)


DOMAIN = "test"


user_account_short_id = "newly_registered"
user_account_full_id = f"{user_account_short_id}@{DOMAIN}"
user_private_key = "1234567890123456789012345678901234567890123456789012345678901234"
user_public_key = IrohaCrypto.derive_public_key(user_private_key).decode("utf-8")

user_private_key_extra = (
    "abcdeabcdeabcdeabcdeabcdeabcdeabcdeabcdeabcdeabcdeabcdeabcdeabcd"
)
user_public_key_extra = IrohaCrypto.derive_public_key(user_private_key_extra).decode(
    "utf-8"
)
user_mail = "john@hyperledger.com"

ASSET_ID = f"lemurcoin#{DOMAIN}"


@integration_helpers.trace
def create_and_setup_account(contract_address: str):
    params = integration_helpers.get_first_four_bytes_of_keccak(
        b"createAndSetupAccount(string,string,string,string,string,string,string,string,string,string)"
    )
    no_of_param = 10
    for x in range(no_of_param):
        params = params + integration_helpers.left_padded_address_of_param(
            x, no_of_param
        )
    params = params + integration_helpers.argument_encoding(
        ADMIN_ACCOUNT_ID
    )  # admin account id
    params = params + integration_helpers.argument_encoding(
        user_account_short_id
    )  # user account name
    params = params + integration_helpers.argument_encoding(
        user_account_full_id
    )  #  user account id
    params = params + integration_helpers.argument_encoding(ASSET_ID)  #  asset id
    params = params + integration_helpers.argument_encoding(DOMAIN)  #  domain name
    params = params + integration_helpers.argument_encoding(
        "set up balance"
    )  #  description
    params = params + integration_helpers.argument_encoding("100")  #  amount
    params = params + integration_helpers.argument_encoding(
        "mail"
    )  #  key for user detail
    params = params + integration_helpers.argument_encoding(
        user_mail
    )  #  value for user detail
    params = params + integration_helpers.argument_encoding(
        user_public_key
    )  #  public key of user
    tx = iroha_admin.transaction(
        [
            iroha_admin.command(
                "CallEngine",
                caller=ADMIN_ACCOUNT_ID,
                callee=contract_address,
                input=params,
            )
        ]
    )
    IrohaCrypto.sign_transaction(tx, ADMIN_PRIVATE_KEY)
    response = net.send_tx(tx)
    for status in net.tx_status_stream(tx):
        print(status)


@integration_helpers.trace
def create_contract():
    bytecode = "608060405234801561001057600080fd5b5073a6abc17819738299b3b2c1ce46d55c74f04e290c6000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550611c1f806100746000396000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c8063b7d66df711610071578063b7d66df7146101c7578063bc53c0c4146101f7578063d4e804ab14610227578063d8742d6114610245578063de58d15614610275578063e048232a146102a5576100b4565b80631b2d95ab146100b95780632c74aaaf146100e95780634bb8f5f5146101195780635bdb3a411461014957806390fe3ac11461016757806395256b2c14610197575b600080fd5b6100d360048036038101906100ce91906112d5565b6102d5565b6040516100e09190611845565b60405180910390f35b61010360048036038101906100fe9190611316565b610441565b6040516101109190611845565b60405180910390f35b610133600480360381019061012e9190611508565b6105b0565b6040516101409190611845565b60405180910390f35b6101516105e9565b60405161015e9190611845565b60405180910390f35b610181600480360381019061017c9190611419565b610748565b60405161018e9190611845565b60405180910390f35b6101b160048036038101906101ac9190611419565b61076c565b6040516101be9190611845565b60405180910390f35b6101e160048036038101906101dc9190611382565b610947565b6040516101ee9190611845565b60405180910390f35b610211600480360381019061020c9190611382565b610b27565b60405161021e9190611845565b60405180910390f35b61022f610cf1565b60405161023c919061182a565b60405180910390f35b61025f600480360381019061025a9190611316565b610d15565b60405161026c9190611845565b60405180910390f35b61028f600480360381019061028a9190611382565b610ed1565b60405161029c9190611845565b60405180910390f35b6102bf60048036038101906102ba9190611316565b6110b1565b6040516102cc9190611845565b60405180910390f35b60606000826040516024016102ea9190611867565b6040516020818303038152906040527f1b2d95ab000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16836040516103b191906117fc565b600060405180830381855af49150503d80600081146103ec576040519150601f19603f3d011682016040523d82523d6000602084013e6103f1565b606091505b509150915081610436576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161042d90611982565b60405180910390fd5b809350505050919050565b606060008383604051602401610458929190611889565b6040516020818303038152906040527f260b5d52000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168360405161051f91906117fc565b600060405180830381855af49150503d806000811461055a576040519150601f19603f3d011682016040523d82523d6000602084013e61055f565b606091505b5091509150816105a4576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161059b906119a2565b60405180910390fd5b80935050505092915050565b60606105bd8a8884610b27565b90506105cc8b8a8a898961076c565b90506105d9898585610947565b90509a9950505050505050505050565b606060006040516024016040516020818303038152906040527f5bdb3a41000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16836040516106ba91906117fc565b600060405180830381855af49150503d80600081146106f5576040519150601f19603f3d011682016040523d82523d6000602084013e6106fa565b606091505b50915091508161073f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161073690611982565b60405180910390fd5b80935050505090565b6060610755868686610ed1565b905061076183836110b1565b905095945050505050565b60606000868686868660405160240161078995949392919061190c565b6040516020818303038152906040527f95256b2c000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168360405161085091906117fc565b600060405180830381855af49150503d806000811461088b576040519150601f19603f3d011682016040523d82523d6000602084013e610890565b606091505b5091509150816108d5576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108cc90611982565b60405180910390fd5b876040516108e39190611813565b6040518091039020896040516108f99190611813565b60405180910390207f6a739057159b3f3e2efcba00d44b0fa47de56972ed8776a2da7682bcf7c67de1876040516109309190611867565b60405180910390a380935050505095945050505050565b60606000848484604051602401610960939291906118c0565b6040516020818303038152906040527fb7d66df7000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1683604051610a2791906117fc565b600060405180830381855af49150503d8060008114610a62576040519150601f19603f3d011682016040523d82523d6000602084013e610a67565b606091505b509150915081610aac576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610aa390611982565b60405180910390fd5b84604051610aba9190611813565b604051809103902086604051610ad09190611813565b604051809103902088604051610ae69190611813565b60405180910390207f5e1b38cd47cf21b75d5051af29fa321eedd94877db5ac62067a076770eddc9d060405160405180910390a48093505050509392505050565b60606000848484604051602401610b40939291906118c0565b6040516020818303038152906040527fbc53c0c4000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1683604051610c0791906117fc565b600060405180830381855af49150503d8060008114610c42576040519150601f19603f3d011682016040523d82523d6000602084013e610c47565b606091505b509150915081610c8c576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c8390611982565b60405180910390fd5b85604051610c9a9190611813565b604051809103902087604051610cb09190611813565b60405180910390207f2681f9ea8419cbc4844760d23371ce7e9bd5686e78f2a7988ff6e8430abd39b860405160405180910390a38093505050509392505050565b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b606060008383604051602401610d2c929190611889565b6040516020818303038152906040527fd8742d61000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1683604051610df391906117fc565b600060405180830381855af49150503d8060008114610e2e576040519150601f19603f3d011682016040523d82523d6000602084013e610e33565b606091505b509150915081610e78576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e6f90611982565b60405180910390fd5b85604051610e869190611813565b60405180910390207f10eebd74a1bfcec9b2e208fad2742f3ce9fde2f1ff95b30db42e2bc6b6b713e386604051610ebd9190611867565b60405180910390a280935050505092915050565b60606000848484604051602401610eea939291906118c0565b6040516020818303038152906040527fde58d156000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1683604051610fb191906117fc565b600060405180830381855af49150503d8060008114610fec576040519150601f19603f3d011682016040523d82523d6000602084013e610ff1565b606091505b509150915081611036576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161102d90611982565b60405180910390fd5b846040516110449190611813565b60405180910390208660405161105a9190611813565b6040518091039020886040516110709190611813565b60405180910390207fe5ab145c34a2b2599d0b309bd4b0141f99353ee85ae41cf5afb5761105b177a860405160405180910390a48093505050509392505050565b6060600083836040516024016110c8929190611889565b6040516020818303038152906040527fe048232a000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168360405161118f91906117fc565b600060405180830381855af49150503d80600081146111ca576040519150601f19603f3d011682016040523d82523d6000602084013e6111cf565b606091505b509150915081611214576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161120b90611982565b60405180910390fd5b856040516112229190611813565b60405180910390207fa6f5bb79d2e3abb706e921433d10213c20741696fef51dd4153f5dc73d9e4d00866040516112599190611867565b60405180910390a280935050505092915050565b600061128061127b846119e7565b6119c2565b90508281526020810184848401111561129857600080fd5b6112a3848285611a98565b509392505050565b600082601f8301126112bc57600080fd5b81356112cc84826020860161126d565b91505092915050565b6000602082840312156112e757600080fd5b600082013567ffffffffffffffff81111561130157600080fd5b61130d848285016112ab565b91505092915050565b6000806040838503121561132957600080fd5b600083013567ffffffffffffffff81111561134357600080fd5b61134f858286016112ab565b925050602083013567ffffffffffffffff81111561136c57600080fd5b611378858286016112ab565b9150509250929050565b60008060006060848603121561139757600080fd5b600084013567ffffffffffffffff8111156113b157600080fd5b6113bd868287016112ab565b935050602084013567ffffffffffffffff8111156113da57600080fd5b6113e6868287016112ab565b925050604084013567ffffffffffffffff81111561140357600080fd5b61140f868287016112ab565b9150509250925092565b600080600080600060a0868803121561143157600080fd5b600086013567ffffffffffffffff81111561144b57600080fd5b611457888289016112ab565b955050602086013567ffffffffffffffff81111561147457600080fd5b611480888289016112ab565b945050604086013567ffffffffffffffff81111561149d57600080fd5b6114a9888289016112ab565b935050606086013567ffffffffffffffff8111156114c657600080fd5b6114d2888289016112ab565b925050608086013567ffffffffffffffff8111156114ef57600080fd5b6114fb888289016112ab565b9150509295509295909350565b6000806000806000806000806000806101408b8d03121561152857600080fd5b60008b013567ffffffffffffffff81111561154257600080fd5b61154e8d828e016112ab565b9a505060208b013567ffffffffffffffff81111561156b57600080fd5b6115778d828e016112ab565b99505060408b013567ffffffffffffffff81111561159457600080fd5b6115a08d828e016112ab565b98505060608b013567ffffffffffffffff8111156115bd57600080fd5b6115c98d828e016112ab565b97505060808b013567ffffffffffffffff8111156115e657600080fd5b6115f28d828e016112ab565b96505060a08b013567ffffffffffffffff81111561160f57600080fd5b61161b8d828e016112ab565b95505060c08b013567ffffffffffffffff81111561163857600080fd5b6116448d828e016112ab565b94505060e08b013567ffffffffffffffff81111561166157600080fd5b61166d8d828e016112ab565b9350506101008b013567ffffffffffffffff81111561168b57600080fd5b6116978d828e016112ab565b9250506101208b013567ffffffffffffffff8111156116b557600080fd5b6116c18d828e016112ab565b9150509295989b9194979a5092959850565b6116dc81611a66565b82525050565b60006116ed82611a18565b6116f78185611a2e565b9350611707818560208601611aa7565b61171081611b3a565b840191505092915050565b600061172682611a18565b6117308185611a3f565b9350611740818560208601611aa7565b80840191505092915050565b600061175782611a23565b6117618185611a4a565b9350611771818560208601611aa7565b61177a81611b3a565b840191505092915050565b600061179082611a23565b61179a8185611a5b565b93506117aa818560208601611aa7565b80840191505092915050565b60006117c3602783611a4a565b91506117ce82611b4b565b604082019050919050565b60006117e6602883611a4a565b91506117f182611b9a565b604082019050919050565b6000611808828461171b565b915081905092915050565b600061181f8284611785565b915081905092915050565b600060208201905061183f60008301846116d3565b92915050565b6000602082019050818103600083015261185f81846116e2565b905092915050565b60006020820190508181036000830152611881818461174c565b905092915050565b600060408201905081810360008301526118a3818561174c565b905081810360208301526118b7818461174c565b90509392505050565b600060608201905081810360008301526118da818661174c565b905081810360208301526118ee818561174c565b90508181036040830152611902818461174c565b9050949350505050565b600060a0820190508181036000830152611926818861174c565b9050818103602083015261193a818761174c565b9050818103604083015261194e818661174c565b90508181036060830152611962818561174c565b90508181036080830152611976818461174c565b90509695505050505050565b6000602082019050818103600083015261199b816117b6565b9050919050565b600060208201905081810360008301526119bb816117d9565b9050919050565b60006119cc6119dd565b90506119d88282611ada565b919050565b6000604051905090565b600067ffffffffffffffff821115611a0257611a01611b0b565b5b611a0b82611b3a565b9050602081019050919050565b600081519050919050565b600081519050919050565b600082825260208201905092915050565b600081905092915050565b600082825260208201905092915050565b600081905092915050565b6000611a7182611a78565b9050919050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b82818337600083830152505050565b60005b83811015611ac5578082015181840152602081019050611aaa565b83811115611ad4576000848401525b50505050565b611ae382611b3a565b810181811067ffffffffffffffff82111715611b0257611b01611b0b565b5b80604052505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6000601f19601f8301169050919050565b7f4572726f722063616c6c696e67207365727669636520636f6e7472616374206660008201527f756e6374696f6e00000000000000000000000000000000000000000000000000602082015250565b7f4572726f722063616c6c696e67207365727669636520636f6e7472616374206660008201527f756e6374696f6e2000000000000000000000000000000000000000000000000060208201525056fea26469706673582212201b922862cbb738686198f77a5011a1092100b5a3470ac612f9e9e8b7d6b8aef664736f6c63430008040033"
    """Bytecode was generated using remix editor  https://remix.ethereum.org/ from file setupAccount.sol. """
    tx = iroha_admin.transaction(
        [iroha_admin.command("CallEngine", caller=ADMIN_ACCOUNT_ID, input=bytecode)]
    )
    IrohaCrypto.sign_transaction(tx, ADMIN_PRIVATE_KEY)
    net.send_tx(tx)
    hex_hash = binascii.hexlify(IrohaCrypto.hash(tx))
    for status in net.tx_status_stream(tx):
        print(status)
    return hex_hash


@integration_helpers.trace
def sets_asset(contract_address: str, asset_id: str, amount: str):
    asset, domain = asset_id.split("#")
    params = integration_helpers.get_first_four_bytes_of_keccak(
        b"setsAsset(string,string,string,string,string)"
    )
    no_of_param = 5
    for x in range(no_of_param):
        params = params + integration_helpers.left_padded_address_of_param(
            x, no_of_param
        )
    params = params + integration_helpers.argument_encoding(asset)  # asset name
    params = params + integration_helpers.argument_encoding(domain)  # domain name
    params = params + integration_helpers.argument_encoding("4")  #  precision
    params = params + integration_helpers.argument_encoding(asset_id)  # asset id
    params = params + integration_helpers.argument_encoding(amount)  # domain name
    tx = iroha_admin.transaction(
        [
            iroha_admin.command(
                "CallEngine",
                caller=ADMIN_ACCOUNT_ID,
                callee=contract_address,
                input=params,
            )
        ]
    )
    IrohaCrypto.sign_transaction(tx, ADMIN_PRIVATE_KEY)
    response = net.send_tx(tx)
    for status in net.tx_status_stream(tx):
        print(status)


def print_paragraph(text: str):
    print(10 * "-", text, ":", 10 * "-")


@integration_helpers.trace
def get_account_details(contract_address: str, user_account: str):
    iroha_user = Iroha(user_account)
    params = integration_helpers.get_first_four_bytes_of_keccak(b"getAccountDetail()")
    no_of_param = 0
    tx = iroha_user.transaction(
        [
            iroha_user.command(
                "CallEngine", caller=user_account, callee=contract_address, input=params
            )
        ]
    )
    IrohaCrypto.sign_transaction(tx, user_private_key)
    response = net.send_tx(tx)
    for status in net.tx_status_stream(tx):
        print(status)
    hex_hash = binascii.hexlify(IrohaCrypto.hash(tx))
    return hex_hash


@integration_helpers.trace
def balance(address: str, user_id: str):
    params = integration_helpers.get_first_four_bytes_of_keccak(
        b"queryBalance(string,string)"
    )
    no_of_param = 2
    for x in range(no_of_param):
        params = params + integration_helpers.left_padded_address_of_param(
            x, no_of_param
        )
    params = params + integration_helpers.argument_encoding(user_id)  # account id
    params = params + integration_helpers.argument_encoding(ASSET_ID)  # asset id
    tx = iroha_admin.transaction(
        [
            iroha_admin.command(
                "CallEngine", caller=ADMIN_ACCOUNT_ID, callee=address, input=params
            )
        ]
    )
    IrohaCrypto.sign_transaction(tx, ADMIN_PRIVATE_KEY)
    response = net.send_tx(tx)
    for status in net.tx_status_stream(tx):
        print(status)
    hex_hash = binascii.hexlify(IrohaCrypto.hash(tx))
    return hex_hash


if __name__ == "__main__":
    print_paragraph("Preparation")
    hash = create_contract()
    address = integration_helpers.get_engine_receipts_address(hash)
    sets_asset(address, ASSET_ID, "10000")

    print_paragraph("Creating account")
    create_and_setup_account(address)

    print_paragraph("Checking account")
    hash = get_account_details(address, user_account_full_id)
    integration_helpers.get_engine_receipts_result(hash)
    hash = balance(address, user_account_full_id)
    integration_helpers.get_engine_receipts_result(hash)