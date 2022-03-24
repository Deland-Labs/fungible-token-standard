@dev
Feature: token burn and burn from

  Background:
    Given Reinstall dft canisters
      | key          | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_mintable | Test Token 3 | TST3   | 16       | 100000000    | 0.2         | 0.00     | 6             | dft_main |
    And owner "dft_main" set "dft_fee_charger" as fee_to

  Scenario:Mint token by not owner will fail
    When "dft_user2" mint 100 "dft_mintable" for "dft_user3" token will fail
    Then Check the dft_mintable balance of dft_user2 should be 0
    And Check the total supply of "dft_mintable" should be "100000000"
    And Check that the fees of "dft_mintable" is "0" by "dft_fee_charger", that means mint does not charge fee

  Scenario:Mint token by owner will success
    When "dft_main" mint 100 "dft_mintable" for "dft_user2" token will success
    Then Check the dft_mintable balance of dft_user2 should be 100
    And Check the total supply of "dft_mintable" should be "100000100"
    And Check that the fees of "dft_mintable" is "0" by "dft_fee_charger", that means mint does not charge fee