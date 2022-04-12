@dft
Feature: token management

  Background:
    Given Reinstall dft canisters
      | key       | name         | symbol | decimals | total_supply | fee_minimum | fee_rate | rate_decimals | owner    |
      | dft_basic | Test Token 1 | TST1   | 18       | 100000000    | 0.01        | 0        | 8             | dft_main |

  Scenario: Update token logo
    When I update token "dft_basic"'s logo "TICP.png" with owner "dft_main", will success
    When I update token "dft_basic"'s logo with invalid image data with owner "dft_main", will failed
    When I update token "dft_basic"'s logo with not owner "dft_user1", will failed

  Scenario: Update token description with not owner will failed
    When I update token "dft_basic"'s description with not owner "dft_user1", will failed
      | key   | value      |
      | DSCVR | test dscvr |

  Scenario: Update token description with owner will success
    When I update token "dft_basic"'s description with owner "dft_main", will success
      | key      | value         |
      | DSCVR    | test dscvr    |
      | OPENCHAT | test openchat |
    Then Get token "dft_basic"'s description by "dft_main",will include blow fields and values
      | key      | value         |
      | DSCVR    | test dscvr    |
      | OPENCHAT | test openchat |

  Scenario: Update token description with owner will success
    When I update token "dft_basic"'s description with owner "dft_main", will success
      | key         | value       |
      | DSCVR       | test dscvr5 |
      | UNKNOWNKYE1 | unknown1    |
      | UNKNOWNKYE1 | unknown1    |
    Then Get token "dft_basic"'s description will not contain "UNKNOWNKYE1" and "UNKNOWNKYE2" by "dft_main"
    Then Get token "dft_basic"'s description by "dft_main",will include blow fields and values
      | key   | value       |
      | DSCVR | test dscvr5 |

  Scenario: Update token fee
    When I update token "dft_basic"'s fee with owner "dft_main", will success
      | minimum | rate | rate_decimals |
      | 0.001   | 0.1  | 8             |
    Then Get token "dft_basic"'s fee by "dft_main",will include blow fields and value
      | minimum | rate | rate_decimals |
      | 0.001   | 0.1  | 8             |
    When I update token "dft_basic"'s fee with not owner "dft_user1", will failed
      | minimum | rate | rate_decimals |
      | 0.001   | 0.1  | 8             |

  Scenario: Update token feeTo
    When I update token "dft_basic"'s feeTo as "dft_user1" with owner "dft_main", will success
    Then Get token "dft_basic"'s feeTo by "dft_main", should be "dft_user1"
    When I update token "dft_basic"'s feeTo as "dft_user2" with not owner "dft_user1", will failed