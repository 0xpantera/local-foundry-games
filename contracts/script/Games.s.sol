// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Script.sol";
import "../src/Game1.sol";
import "../src/Game2.sol";
import "../src/Game3.sol";
import "../src/Game4.sol";

contract Game1Script is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        Game1 game = new Game1();
        vm.stopBroadcast();
    }
}

contract Game2Script is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        Game2 game = new Game2();
        vm.stopBroadcast();
    }
}

contract Game3Script is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        Game3 game = new Game3();
        vm.stopBroadcast();
    }
}

contract Game4Script is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        Game4 game = new Game4();
        vm.stopBroadcast();
    }
}