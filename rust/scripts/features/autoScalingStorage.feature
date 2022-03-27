@dft
Feature: token tx auto scaling storage

  Background:
    Given Reinstall dft canisters
      | key       | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_basic | Test Token 1 | TST1   | 18       | 100000000    | 0.01        | 0        | 8             | dft_main |
    And owner "dft_main" set "dft_fee_charger" as fee_to

  Scenario:Auto scaling storage when tx count is more than 2000 times
    When Check the storage canisters count is equal to "0" ,by "dft_main"
    Then Transfer token "dft_basic" from "dft_main" to "dft_user1" amount of equals the times, repeat 2010 times
    Then Check the storage canisters count is equal to "1" ,by "dft_main"
    Then Check the tx index "1999" transfer transaction of "dft_basic", the amount should be 1998
    Then Check the tx index "999" transfer transaction of "dft_basic", the result should be a forward result
    Then Check the tx index "1000" transfer transaction of "dft_basic", the result should not be a forward result
    Then Check the last 10 transactions of "dft_basic", check each transaction is correct
      | index | caller   | from     | to        | amount | fee  | nonce |
      | 2011  | dft_main | dft_main | dft_user1 | 2010   | 0.01 | 2011  |
      | 2010  | dft_main | dft_main | dft_user1 | 2009   | 0.01 | 2010  |