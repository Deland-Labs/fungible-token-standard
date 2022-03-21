@dev
Feature: token transfer

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
    And Check that the transfer fees of <token> by <diff> charged fee is <fee>,fee to is <fee_to>
    Examples:
      | spender   | owner     | receiver     | fee_to          | diff | token        | amountOwner | amountSpender | amountReceiver | fee     |
      | dft_user1 | dft_miner | dft_receiver | dft_fee_charger | 100  | dft_basic    | 99899.98    | 0             | 100            | 0.03    |
      | dft_user2 | dft_user1 | dft_receiver | dft_fee_charger | 100  | dft_basic2   | 99899.989   | 0             | 100            | 10.011  |
      | dft_user3 | dft_user2 | dft_receiver | dft_fee_charger | 100  | dft_burnable | 99897.8     | 0             | 100            | 2002.2  |
      | dft_miner | dft_user3 | dft_receiver | dft_fee_charger | 100  | dft_mintable | 99889.7     | 0             | 100            | 10010.3 |