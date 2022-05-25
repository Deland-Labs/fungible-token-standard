@dev
Feature: token transfer

  Background:
    Given Reinstall dft canisters
      | key       | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_basic | Test Token 1 | TST1   | 18       | 100000000    | 0.01        | 0        | 8             | dft_main |
    And owner "dft_main" set "dft_fee_charger" as fee_to
    And transfer token from "dft_main" to these users
      | token     | user      | amount  |
      | dft_basic | dft_miner | 100000 |

  Scenario Outline:Transfer token to a receiver
    When <userA> transfer <diff> <token> to <canister> immediate
    Then Check the <token> balance of <userA> should be <amountA>
    And Check the <token> balance of <canister> should be <amountB>
    And Check receiver's notification count should be "1"
    Then <userA> transfer <diff> <token> to <canister> immediate
    And Check receiver's notification count should be "2"
    Examples:
      | userA     | canister     | diff | token     | amountA  | amountB |
      | dft_miner | dft_receiver | 100  | dft_basic | 99899.99 | 100     |