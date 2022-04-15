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

  Scenario Outline:Transfer token from the owner to a receiver
    When <spender> transfer from <owner> to <receiver>,<diff> <token>
    Then Check the <token> balance of <owner> should be <amountOwner>
    And Check the <token> balance of <spender> should be <amountSpender>
    And Check the <token> balance of <receiver> should be <amountReceiver>
    And Check that the transfer fees of <token> by <diff> charged fee is <fee>,fee to is <feeTo>
    Examples:
      | spender   | owner     | receiver     | feeTo           | diff | token        | amountOwner | amountSpender | amountReceiver | fee     |
      | dft_user1 | dft_miner | dft_receiver | dft_fee_charger | 100  | dft_basic    | 99899.98    | 0             | 100            | 0.03    |
      | dft_user2 | dft_user1 | dft_receiver | dft_fee_charger | 100  | dft_basic2   | 99899.989   | 0             | 100            | 10.011  |
      | dft_user3 | dft_user2 | dft_receiver | dft_fee_charger | 100  | dft_burnable | 99897.8     | 0             | 100            | 2002.2  |
      | dft_miner | dft_user3 | dft_receiver | dft_fee_charger | 100  | dft_mintable | 99889.7     | 0             | 100            | 10010.3 |

  @dev
  Scenario:Approve from the owner to a spender
    When "dft_miner" approve "dft_basic" to "dft_user1", "1"
    Then Check the "dft_basic" allowance of "dft_miner" "dft_user1" should be "1"
    Then "dft_miner" approve "dft_basic" to "dft_user1", "5"
    And Check the "dft_basic" allowance of "dft_miner" "dft_user1" should be "5"

  Scenario:Approve not enough for transfer
    When "dft_miner" approve "dft_basic" to "dft_user1", "100"
    Then "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "100" will failed
    When "dft_miner" approve "dft_basic" to "dft_user1", "100.1"
    Then "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "100" will success
    And Check the "dft_basic" allowance of "dft_miner" "dft_user1" should be "0.09"
    And Check the dft_basic balance of dft_user1 should be 0
    And Check the dft_basic balance of dft_user2 should be 100

  Scenario:Approve twice with same property will fail
    When "dft_miner" approve "dft_basic" to "dft_user1", "100" twice , the second will failed

  Scenario:Approve with pass 1 day will fail
    When "dft_miner" approve "dft_basic" to "dft_user1", "100" with timestamp passed "1" day, will failed

  Scenario:TransferFrom twice with same property will fail
    When "dft_miner" approve "dft_basic" to "dft_user1", "200"
    And "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "100" twice, the second will failed

  Scenario:TransferFrom with pass 1 day will fail
    When  "dft_miner" approve "dft_basic" to "dft_user1", "200"
    And "dft_user1" transfer "dft_basic" from "dft_miner" to "dft_user2" "100" with timestamp passed "1" day, will failed
