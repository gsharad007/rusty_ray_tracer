Feature: Animal feature
  @hungry
  Scenario: If we feed a hungry cat it will no longer be hungry
    Given a hungry cat
    When I feed the cat 1 times
    Then the cat is not hungry

  @satiated @second
  Scenario: If we feed a satiated cat it will not become hungry
    Given a satiated cat
    When I feed the cat 1 times
    Then the cat is not hungry

  @satiated @third
  Scenario: If we feed a satiated cat too much it will explode
    Given a satiated cat
    When I feed the cat 2 times
    Then the cat has exploded

  @full @fourth
  Scenario: If we feed a full cat it will explode
    Given a full cat
    When I feed the cat 1 times
    Then the cat has exploded
