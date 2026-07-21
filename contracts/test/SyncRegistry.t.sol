// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {SyncRegistry} from "../src/SyncRegistry.sol";

contract SyncRegistryTest is Test {
    SyncRegistry public registry;

    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");

    string constant CID_V1 = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi";
    string constant CID_V2 = "bafybeidmur3iutbzsbapbfm3rvqk7bvz3uqydxsqhsgfk2rq3vbmkagmq";

    bytes32 constant HASH_V1 = keccak256("sync-content-v1");
    bytes32 constant HASH_V2 = keccak256("sync-content-v2");

    function setUp() public {
        registry = new SyncRegistry();
    }

    // -------------------------------------------------------------------------
    // updateRecord — caminho feliz
    // -------------------------------------------------------------------------

    function test_UpdateRecord_Success() public {
        vm.prank(alice);
        registry.updateRecord(CID_V1, HASH_V1);

        SyncRegistry.CidRecord memory rec = registry.getRecord(alice);
        assertEq(rec.cid, CID_V1);
        assertEq(rec.contentHash, HASH_V1);
        assertEq(rec.version, 1);
        assertEq(rec.updatedAt, block.timestamp);
        assertTrue(rec.exists);
    }

    function test_UpdateRecord_EmiteEvento() public {
        vm.prank(alice);
        vm.expectEmit(true, true, false, true);
        emit SyncRegistry.RecordUpdated(alice, CID_V1, HASH_V1, 1);

        registry.updateRecord(CID_V1, HASH_V1);
    }

    function test_UpdateRecord_SegundaAtualizacaoIncrementaVersao() public {
        vm.prank(alice);
        registry.updateRecord(CID_V1, HASH_V1);

        vm.warp(block.timestamp + 60);

        vm.prank(alice);
        registry.updateRecord(CID_V2, HASH_V2);

        SyncRegistry.CidRecord memory rec = registry.getRecord(alice);
        assertEq(rec.cid, CID_V2);
        assertEq(rec.contentHash, HASH_V2);
        assertEq(rec.version, 2);
    }

    function test_UpdateRecord_NaoAfetaOutrosEnderecos() public {
        vm.prank(alice);
        registry.updateRecord(CID_V1, HASH_V1);

        assertFalse(registry.hasRecord(bob));
    }

    // -------------------------------------------------------------------------
    // updateRecord — erros
    // -------------------------------------------------------------------------

    function test_Revert_UpdateRecord_CidVazio() public {
        vm.prank(alice);
        vm.expectRevert(SyncRegistry.EmptyCid.selector);
        registry.updateRecord("", HASH_V1);
    }

    function test_Revert_UpdateRecord_ContentHashVazio() public {
        vm.prank(alice);
        vm.expectRevert(SyncRegistry.EmptyContentHash.selector);
        registry.updateRecord(CID_V1, bytes32(0));
    }

    // -------------------------------------------------------------------------
    // getRecord
    // -------------------------------------------------------------------------

    function test_Revert_GetRecord_NaoEncontrado() public {
        vm.expectRevert(abi.encodeWithSelector(SyncRegistry.RecordNotFound.selector, bob));
        registry.getRecord(bob);
    }

    // -------------------------------------------------------------------------
    // hasRecord
    // -------------------------------------------------------------------------

    function test_HasRecord_FalseAntesDeAtualizar() public view {
        assertFalse(registry.hasRecord(alice));
    }

    function test_HasRecord_TrueAposAtualizar() public {
        vm.prank(alice);
        registry.updateRecord(CID_V1, HASH_V1);

        assertTrue(registry.hasRecord(alice));
    }

    // -------------------------------------------------------------------------
    // Isolamento entre endereços
    // -------------------------------------------------------------------------

    function test_DoisEnderecos_RegistrosSeparados() public {
        vm.prank(alice);
        registry.updateRecord(CID_V1, HASH_V1);

        vm.prank(bob);
        registry.updateRecord(CID_V2, HASH_V2);

        SyncRegistry.CidRecord memory aliceRec = registry.getRecord(alice);
        SyncRegistry.CidRecord memory bobRec = registry.getRecord(bob);

        assertEq(aliceRec.cid, CID_V1);
        assertEq(bobRec.cid, CID_V2);
        assertEq(aliceRec.version, 1);
        assertEq(bobRec.version, 1);
    }
}
