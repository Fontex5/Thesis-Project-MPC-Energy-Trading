package examples;

import com.partisiablockchain.BlockchainAddress;
import com.partisiablockchain.language.abicodegen.ZkDoubleAuction;
import com.partisiablockchain.language.junit.ContractBytes;
import com.partisiablockchain.language.junit.ContractTest;
import com.partisiablockchain.language.junit.JunitContractTest;
import com.partisiablockchain.language.junit.exceptions.ActionFailureException;
import com.partisiablockchain.language.testenvironment.zk.node.task.PendingInputId;
import com.partisiablockchain.language.testenvironment.zk.node.task.PendingOpenId;
import com.secata.stream.BitOutput;
import com.secata.stream.CompactBitArray;
import java.nio.file.Path;
import java.util.List;
import org.assertj.core.api.Assertions;

public final class ZkDoubleAuctionTest extends JunitContractTest{
    private static final ContractBytes ZK_DOUBLE_AUCTION_BYTES = 
          ContractBytes.fromPaths(
          Path.of("../rust/target/wasm32-unknown-unknown/release/zk_double_auction.zkwa"),
          Path.of("../rust/target/wasm32-unknown-unknown/release/zk_double_auction.abi"),
          Path.of("../rust/target/wasm32-unknown-unknown/release/zk_double_auction_runner"));

    private BlockchainAddress household1;
    private BlockchainAddress household2;
    private BlockchainAddress household3;
    private BlockchainAddress household4;
    private BlockchainAddress household5;
    private BlockchainAddress household6;
    private BlockchainAddress double_auction_contract;

    @ContractTest
    void deploy() {
        household1 = blockchain.newAccount(2);
        household2 = blockchain.newAccount(3);
        household3 = blockchain.newAccount(4);
        household4 = blockchain.newAccount(5);
        household5 = blockchain.newAccount(6);
        household6 = blockchain.newAccount(7);

        byte[] initRpc = ZkDoubleAuction.initialize();

        double_auction_contract = blockchain.deployZkContract(household1, ZK_DOUBLE_AUCTION_BYTES, initRpc);

        final var contractByteState = blockchain.getContractState(double_auction_contract);

        ZkDoubleAuction.ContractState state = 
            ZkDoubleAuction.ContractState.deserialize(blockchain.getContractState(double_auction_contract));

        Assertions.assertThat(state).isNotNull();
        Assertions.assertThat(state.equilibriumPrice()).isNull();
        Assertions.assertThat(state.matchedOrders()).isEmpty();
    }

    @ContractTest(previous = "deploy")
    void equilibriumPriceInMiddle() {

        DoubleAuctionOrder sell_order1 = new DoubleAuctionOrder(1,0,0,0,4,0,0);
        DoubleAuctionOrder sell_order2 = new DoubleAuctionOrder(2,0,0,0,5,2,0);

        DoubleAuctionOrder buy_order1 = new DoubleAuctionOrder(3,0,0,3,0,0,0);
        DoubleAuctionOrder buy_order2 = new DoubleAuctionOrder(4,0,0,3,2,0,0);
        DoubleAuctionOrder buy_order3 = new DoubleAuctionOrder(5,5,0,0,0,0,0);
        DoubleAuctionOrder buy_order4 = new DoubleAuctionOrder(6,0,0,0,4,0,0);
        DoubleAuctionOrder buy_order5 = new DoubleAuctionOrder(7,0,2,0,1,1,0);

        blockchain.sendSecretInput(double_auction_contract, household2, createSecretInput(sell_order1), secretInputSellingRpc()); 
        blockchain.sendSecretInput(double_auction_contract, household3, createSecretInput(sell_order2), secretInputSellingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order1), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order2), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order3), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order4), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order5), secretInputBuyingRpc());

        byte[] findEqPrice = ZkDoubleAuction.holdDoubleAuction();

        blockchain.sendAction(household1, double_auction_contract, findEqPrice);

        ZkDoubleAuction.ContractState state = ZkDoubleAuction.ContractState.deserialize(blockchain.getContractState(double_auction_contract));

        Assertions.assertThat(state.equilibriumPrice()).isEqualTo(3);
        Assertions.assertThat(state.matchedOrders().size()).isEqualTo(10);
        Assertions.assertThat(state.matchedOrders().get(0)).isEqualTo(new ZkDoubleAuction.TradeResult(3,1,0));
        Assertions.assertThat(state.matchedOrders().get(9)).isEqualTo(new ZkDoubleAuction.TradeResult(7,2,1));
    }

    @ContractTest(previous = "deploy")
    void equilibriumPriceInLeft() {

        DoubleAuctionOrder sell_order1 = new DoubleAuctionOrder(1,6,0,0,0,0,0);
        DoubleAuctionOrder sell_order2 = new DoubleAuctionOrder(2,2,4,0,0,0,0);

        DoubleAuctionOrder buy_order1 = new DoubleAuctionOrder(3,3,4,0,0,0,0);
        DoubleAuctionOrder buy_order2 = new DoubleAuctionOrder(4,2,0,5,0,0,0);
        DoubleAuctionOrder buy_order3 = new DoubleAuctionOrder(5,0,0,0,0,4,0);
        DoubleAuctionOrder buy_order4 = new DoubleAuctionOrder(6,0,0,0,0,0,0);
        DoubleAuctionOrder buy_order5 = new DoubleAuctionOrder(7,3,0,0,0,0,0);

        blockchain.sendSecretInput(double_auction_contract, household2, createSecretInput(sell_order1), secretInputSellingRpc()); 
        blockchain.sendSecretInput(double_auction_contract, household3, createSecretInput(sell_order2), secretInputSellingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order1), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order2), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order3), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order4), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order5), secretInputBuyingRpc());

        byte[] findEqPrice = ZkDoubleAuction.holdDoubleAuction();

        blockchain.sendAction(household1, double_auction_contract, findEqPrice);

        ZkDoubleAuction.ContractState state = ZkDoubleAuction.ContractState.deserialize(blockchain.getContractState(double_auction_contract));

        Assertions.assertThat(state.equilibriumPrice()).isEqualTo(0);
        Assertions.assertThat(state.matchedOrders().size()).isEqualTo(10);
        Assertions.assertThat(state.matchedOrders().get(0)).isEqualTo(new ZkDoubleAuction.TradeResult(3,1,3));
        Assertions.assertThat(state.matchedOrders().get(9)).isEqualTo(new ZkDoubleAuction.TradeResult(7,2,2));
    }

    @ContractTest(previous = "deploy")
    void equilibriumPriceInRight() {

        DoubleAuctionOrder sell_order1 = new DoubleAuctionOrder(1,0,0,0,0,2,4);
        DoubleAuctionOrder sell_order2 = new DoubleAuctionOrder(2,0,0,0,0,0,5);

        DoubleAuctionOrder buy_order1 = new DoubleAuctionOrder(3,0,0,0,0,0,3);
        DoubleAuctionOrder buy_order2 = new DoubleAuctionOrder(4,0,0,0,0,2,0);
        DoubleAuctionOrder buy_order3 = new DoubleAuctionOrder(5,5,0,0,0,0,0);
        DoubleAuctionOrder buy_order4 = new DoubleAuctionOrder(6,0,4,0,0,0,0);
        DoubleAuctionOrder buy_order5 = new DoubleAuctionOrder(7,0,0,3,0,0,2);

        blockchain.sendSecretInput(double_auction_contract, household2, createSecretInput(sell_order1), secretInputSellingRpc()); 
        blockchain.sendSecretInput(double_auction_contract, household3, createSecretInput(sell_order2), secretInputSellingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order1), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order2), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order3), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order4), secretInputBuyingRpc());
        blockchain.sendSecretInput(double_auction_contract, household4, createSecretInput(buy_order5), secretInputBuyingRpc());

        byte[] findEqPrice = ZkDoubleAuction.holdDoubleAuction();

        blockchain.sendAction(household1, double_auction_contract, findEqPrice);

        ZkDoubleAuction.ContractState state = ZkDoubleAuction.ContractState.deserialize(blockchain.getContractState(double_auction_contract));

        Assertions.assertThat(state.equilibriumPrice()).isEqualTo(5);
        Assertions.assertThat(state.matchedOrders().size()).isEqualTo(10);
        Assertions.assertThat(state.matchedOrders().get(0)).isEqualTo(new ZkDoubleAuction.TradeResult(3,1,3));
        Assertions.assertThat(state.matchedOrders().get(9)).isEqualTo(new ZkDoubleAuction.TradeResult(7,2,1));
    }

    private record DoubleAuctionOrder(int houseId, int price, int price2, int price3, int price4, int price5, int price6){}

    private CompactBitArray createSecretInput(DoubleAuctionOrder order) {
        return BitOutput.serializeBits(
            bitOutput -> {
            bitOutput.writeUnsignedInt(order.houseId, 32);
            bitOutput.writeUnsignedInt(order.price, 32);
            bitOutput.writeUnsignedInt(order.price2, 32);
            bitOutput.writeUnsignedInt(order.price3, 32);
            bitOutput.writeUnsignedInt(order.price4, 32);
            bitOutput.writeUnsignedInt(order.price5, 32);
            bitOutput.writeUnsignedInt(order.price6, 32);
        });
    }

    byte[] secretInputBuyingRpc() {
        return new byte[] {0x40};
    }

    byte[] secretInputSellingRpc() {
        return new byte[] {0x45};
    }
}