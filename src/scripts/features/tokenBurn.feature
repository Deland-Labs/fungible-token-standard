@dft
Feature: token burn and burn from

  Background:
    Given Reinstall dft canisters
      | key          | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_burnable | Test Token 3 | TST3   | 16       | 100000000    | 0.2         | 0.00     | 6             | dft_main |
    And owner "dft_main" set "dft_fee_charger" as fee_to
    And transfer token from "dft_main" to these users
      | token        | user      | amount |
      | dft_burnable | dft_user2 | 100000 |

  Scenario:Burn token
    When "dft_user2" burn 100 "dft_burnable" token
    Then Check the dft_burnable balance of dft_user2 should be 99900
    And Check that the fees of "dft_burnable" is "0.2" by "dft_fee_charger", that means burn does not charge fee

  Scenario:Burn token will failed when balance not enough
    When "dft_user2" burn "100001" "dft_burnable" token will failed
    Then Check the dft_burnable balance of dft_user2 should be 100000
    And Check that the fees of "dft_burnable" is "0.2" by "dft_fee_charger", that means burn does not charge fee

  Scenario:Burn token too small will be failed
    When "dft_user2" burn "0.1" "dft_burnable" token will failed
    Then Check the dft_burnable balance of dft_user2 should be 100000
    And Check that the fees of "dft_burnable" is "0.2" by "dft_fee_charger", that means burn does not charge fee

  Scenario:Burn from token will failed when allowance not enough
    When "dft_user2" approve "dft_burnable" to "dft_user1", "1"
    Then "dft_user1" burn "2" from "dft_user2" "dft_burnable" token will failed
    Then Check the dft_burnable balance of dft_user2 should be 99999.8
    And Check the "dft_burnable" allowance of "dft_user2" "dft_user1" should be "1"
    And Check that the fees of "dft_burnable" is "0.4" by "dft_fee_charger", that means burn does not charge fee

  Scenario:Burn from token will success when allowance enough
    When "dft_user2" approve "dft_burnable" to "dft_user1", "2"
    Then "dft_user1" burn "2" from "dft_user2" "dft_burnable" token will sucess
    Then Check the dft_burnable balance of dft_user2 should be 99997.8
    And Check the "dft_burnable" allowance of "dft_user2" "dft_user1" should be "0"
    And Check that the fees of "dft_burnable" is "0.4" by "dft_fee_charger", that means burn does not charge fee
