@dev
Feature: token transfer

  Background:
    Given Reinstall dft canisters
      | key       | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_basic | Test Token 1 | TST1   | 18       | 100000000    | 0.01        | 0        | 8             | dft_main |

  Scenario: Update token description with not owner will failed
    When I update token "dft_basic"'s description with not owner("dft_user1"), will failed
      | key   | value      |
      | DSCVR | test dscvr |

  Scenario: Update token description with owner will success
    When I update token "dft_basic"'s description with owner("dft_main"), will success
      | key      | value         |
      | DSCVR    | test dscvr    |
      | OPENCHAT | test openchat |
    Then Get token "dft_basic"'s description by "dft_main",will include blow fields and values
      | key      | value         |
      | DSCVR    | test dscvr    |
      | OPENCHAT | test openchat |

  Scenario: Update token description with owner will success
    When I update token "dft_basic"'s description with owner("dft_main"), will success
      | key         | value       |
      | DSCVR       | test dscvr5 |
      | UNKNOWNKYE1 | unknown1    |
      | UNKNOWNKYE1 | unknown1    |
    Then Get token "dft_basic"'s description will not contain "UNKNOWNKYE1" and "UNKNOWNKYE2" by "dft_main"

    Then Get token "dft_basic"'s description by "dft_main",will include blow fields and values
      | key   | value       |
      | DSCVR | test dscvr5 |
