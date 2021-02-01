Feature: An example feature
  Scenario: Get Room object ID 'foobar'
    Given Room with ID 'foobar'
    Then Room with ID 'foobar' should exist in the BrowserWorld

  Scenario: Get Room object ID '1'
    Given Room with ID '1'
    Then Room with ID '1' should exist in the BrowserWorld

  Scenario: Alice and Bob receives 'on_new_remote_track' callback
    Given Member Alice
    And Alice publishes audio and video to the Bob
    Given Member Bob
    And Bob publishes audio and video to the Alice
    Then Alice's 'on_new_remote_track' callback fired
    And Bob's 'on_new_remote_track' callback fired

  Scenario: Alice disables video for Bob
    Given Member Alice
    And Member Bob
    Given Alice publishes video to the Bob
    When Alice disables video
    Then Bob's 'on_track_disable' callback fired for video track

  Scenario: Alice disable audio for Bob
    Given Member Alice
    And Member Bob
    Given Alice publishes audio to the Bob
    When Alice disables video
    Then Bob's 'on_track_disable' callback fired for audio track

  Scenario: Alice enabled audio for Bob
    Given Member Alice
    And Member Bob
    Given Alice publishes audio to the Bob
    And Alice's audio is disabled
    When Alice enables audio
    Then Bob's 'on_track_enabled' callback fired for audio track

  Scenario: Alice leaves Room
    Given Member Alice
    When Alice leaves Room
    Then Control API receives OnLeave callback for Alice