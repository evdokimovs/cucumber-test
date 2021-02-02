Feature: An example feature
  Scenario: Get Room object ID 'foobar'
    Given Room with ID 'foobar'
    Then Room with ID 'foobar' should exist in the BrowserWorld

    Scenario: Room.on_new_connection callback fires on interconnection
      Given Member Alice
      And Member Bob
      When Alice joins Room
      And Bob joins Room
      Then Alice's 'on_new_connection' callback fires
      And Bob's 'on_new_connection' callback fires
