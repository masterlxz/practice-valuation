// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";
import {SyncRegistry} from "../src/SyncRegistry.sol";

contract DeploySyncRegistry is Script {
    function run() external {
        vm.startBroadcast();

        SyncRegistry registry = new SyncRegistry();

        vm.stopBroadcast();

        console.log("SyncRegistry     :", address(registry));
    }
}
