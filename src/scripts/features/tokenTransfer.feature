@dft
Feature: token transfer

  Background:
    Given Reinstall dft canisters
      | key                   | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_basic             | Test Token 1 | TST1   | 18       | 100000000    | 0.01        | 0        | 8             | dft_main |
      | dft_all_features | Test Token 2 | TST2   | 8        | 100000000    | 0.001       | 0.0001   | 4             | dft_main |
      | dft_burnable          | Test Token 3 | TST3   | 16       | 100000000    | 0.2         | 0.02     | 6             | dft_main |
      | dft_mintable          | Test Token 4 | TST4   | 12       | 100000000    | 0.3         | 0.1      | 5             | dft_main |
    And owner "dft_main" set "dft_fee_charger" as fee_to
    And transfer token from "dft_main" to these users
      | token                 | user      | amount |
      | dft_basic             | dft_miner | 100000 |
      | dft_all_features | dft_user1 | 100000 |
      | dft_burnable          | dft_user2 | 100000 |
      | dft_mintable          | dft_user3 | 100000 |

  Scenario Outline:Transfer token to a receiver
    When <userA> transfer <diff> <token> to <userB> immediate
    Then Check the <token> balance of <userA> should be <amountA>
    And Check the <token> balance of <userB> should be <amountB>
    And Check that the transfer fees of <token> by <diff> charged fee is <fee>,fee to is <feeTo>
    Examples:
      | userA     | userB        | feeTo           | diff | token                 | amountA  | amountB | fee     |
      | dft_miner | dft_receiver | dft_fee_charger | 100  | dft_basic             | 99899.99 | 100     | 0.02    |
      | dft_user1 | dft_receiver | dft_fee_charger | 100  | dft_all_features | 99899.99 | 100     | 10.01   |
      | dft_user2 | dft_receiver | dft_fee_charger | 100  | dft_burnable          | 99898    | 100     | 2002.00 |
      | dft_user3 | dft_receiver | dft_fee_charger | 100  | dft_mintable          | 99890    | 100     | 10010   |

  Scenario:Transfer twice with same property will fail
    When "dft_miner" transfer "100" "dft_basic" to "dft_receiver" twice, the second will fail

  Scenario:Transfer passed 1 days will fail
    When "dft_miner" transfer "100" "dft_basic" to "dft_receiver" passed "1" days will fail
