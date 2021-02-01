Feature: An example feature
  Scenario: Get Room object ID 'foobar'
    Given Room with ID 'foobar'
    Then Room with ID 'foobar' should exist in the BrowserWorld

  Scenario: Get Room object ID '1'
    Given Room with ID '1'
    Then Room with ID '1' should exist in the BrowserWorld