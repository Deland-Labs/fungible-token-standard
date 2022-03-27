@dft
Feature: token approve and transfer from

  Background:
    Given Reinstall dft canisters
      | key          | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_basic    | Test Token 1 | TST1   | 18       | 100000000    | 0.01        | 0        | 8             | dft_main |
      | dft_basic2   | Test Token 2 | TST2   | 8        | 100000000    | 0.001       | 0.0001   | 4             | dft_main |
      | dft_burnable | Test Token 3 | TST3   | 16       | 100000000    | 0.2         | 0.02     | 6             | dft_main |
      | dft_mintable | Test Token 4 | TST4   | 12       | 100000000    | 0.3         | 0.1      | 5             | dft_main |
    And owner "dft_main" set "dft_fee_charger" as fee_to
    And transfer token from "dft_main" to these users
      | token        | user      | amount |
      | dft_basic    | dft_miner | 100000 |
      | dft_basic2   | dft_user1 | 100000 |
      | dft_burnable | dft_user2 | 100000 |
      | dft_mintable | dft_user3 | 100000 |
    And approve tokens from owner to spender in table
      | token        | owner     | spender   | amount |
      | dft_basic    | dft_miner | dft_user1 | 100000 |
      | dft_basic2   | dft_user1 | dft_user2 | 100000 |
      | dft_burnable | dft_user2 | dft_user3 | 100000 |
      | dft_mintable | dft_user3 | dft_miner | 100000 |

  Scenario:Approve for nonce verification
    When "dft_miner" approve "dft_basic" to "dft_user1", "1" with wrong nonce , will fail
    When "dft_miner" approve "dft_basic" to "dft_user1", "1" with out nonce , the nonce should increase 1
    When "dft_miner" approve "dft_basic" to "dft_user1", "1" with correct nonce , the nonce should increase 1

  Scenario:Transfer from for nonce verification
    When "dft_miner" approve "dft_basic" to "dft_user1", "100.1"
    Then "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "110" will fail, the nonce will not change
    Then "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "100" with wrong nonce will fail, the nonce will not change
    Then "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "100" without nonce will success, the nonce should increase 1
    When "dft_miner" approve "dft_basic" to "dft_user1", "100.1"
    Then "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "100" with correct nonce will success, the nonce should increase 1

  Scenario:Transfer for nonce verification
    When "dft_miner" transfer "10000000000" "dft_basic" to "dft_user1" with out nonce,will fail, the nonce will not change
    When "dft_miner" transfer "100" "dft_basic" to "dft_user1" with wrong nonce,will fail, the nonce will not change
    When "dft_miner" transfer "100" "dft_basic" to "dft_user1" with out nonce, the nonce should increase 1
    When "dft_miner" transfer "100" "dft_basic" to "dft_user1" with correct nonce, the nonce should increase 1

  Scenario: Update token logo for nonce verification
    When I update token "dft_basic"'s logo "TICP.png" with owner "dft_main" with wrong nonce, will fail, the nonce will not change
    When I update token "dft_basic"'s logo "TICP.png" with owner "dft_main" with out nonce, the nonce should increase 1
    When I update token "dft_basic"'s logo "TICP.png" with owner "dft_main" with correct nonce, the nonce should increase 1

  Scenario: Update token description for nonce verification
    When I update token "dft_basic"'s description with not owner "dft_user1", will fail, the nonce will not change
      | key   | value      |
      | DSCVR | test dscvr |
    When I update token "dft_basic"'s description with owner "dft_main" with wrong nonce, will fail, the nonce will not change
      | key      | value          |
      | DSCVR    | test dscvr1    |
      | OPENCHAT | test openchat1 |
    When I update token "dft_basic"'s description with owner "dft_main" with out nonce, the nonce should increase 1
      | key      | value          |
      | DSCVR    | test dscvr2    |
      | OPENCHAT | test openchat2 |
    When I update token "dft_basic"'s description with owner "dft_main" with correct nonce, the nonce should increase 1
      | key      | value          |
      | DSCVR    | test dscvr3    |
      | OPENCHAT | test openchat3 |

  Scenario: Update token fee for nonce verification
    When I update token "dft_basic"'s fee with owner "dft_main" with wrong nonce, will fail, the nonce will not change
      | minimum | rate | rate_decimals |
      | 0.001   | 0.1  | 8             |
    When I update token "dft_basic"'s fee with owner "dft_main" with out nonce, the nonce should increase 1
      | minimum | rate | rate_decimals |
      | 0.001   | 0.1  | 8             |
    Then Get token "dft_basic"'s fee by "dft_main",will include blow fields and value
      | minimum | rate | rate_decimals |
      | 0.001   | 0.1  | 8             |
    When I update token "dft_basic"'s fee with owner "dft_main" with correct nonce, the nonce should increase 1
      | minimum | rate | rate_decimals |
      | 0.002   | 0.2  | 6             |
    Then Get token "dft_basic"'s fee by "dft_main",will include blow fields and value
      | minimum | rate | rate_decimals |
      | 0.002   | 0.2  | 6             |

  Scenario: Update token feeTo for nonce verification
    When I update token "dft_basic"'s feeTo as "dft_user1" with owner "dft_main" with wrong nonce, will fail, the nonce will not change
    When I update token "dft_basic"'s feeTo as "dft_user1" with owner "dft_main" with out nonce, the nonce should increase 1
    Then Get token "dft_basic"'s feeTo by "dft_main", should be "dft_user1"
    When I update token "dft_basic"'s feeTo as "dft_user2" with owner "dft_main" with correct nonce, the nonce should increase 1
    Then Get token "dft_basic"'s feeTo by "dft_main", should be "dft_user2"

  Scenario:Burn token for nonce verification
    When "dft_user2" burn 100 "dft_burnable" token with wrong nonce, will fail, the nonce will not change
    When "dft_user2" burn 100 "dft_burnable" token with out nonce, the nonce should increase 1
    When "dft_user2" burn 100 "dft_burnable" token with correct nonce, the nonce should increase 1

  Scenario:Burn from token for nonce verification
    When "dft_user2" approve "dft_burnable" to "dft_user1", "100.1"
    Then "dft_user1" burn "50" from "dft_user2" "dft_burnable" token with wrong nonce, will fail, the nonce will not change
    When "dft_user1" burn "50" from "dft_user2" "dft_burnable" token with out nonce, the nonce should increase 1
    When "dft_user1" burn "50" from "dft_user2" "dft_burnable" token with correct nonce, the nonce should increase 1

  Scenario:Mint token for nonce verification
    When "dft_main" mint 100 "dft_mintable" for "dft_user2" token with wrong nonce, will fail, the nonce will not change
    When "dft_main" mint 100 "dft_mintable" for "dft_user2" token with out nonce, the nonce should increase 1
    When "dft_main" mint 100 "dft_mintable" for "dft_user2" token with correct nonce, the nonce should increase 1