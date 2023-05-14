# `periodic_task`

This crate provides a structure for a long-lived task which is meant to be run periodically. It is synchronous, and the task may be canceled.

Potential use cases might include:

- timing out stale connections or sessions
- periodically updating state and publishing a message over PubSub
- reading a value from a sensor at a prescribed rate
