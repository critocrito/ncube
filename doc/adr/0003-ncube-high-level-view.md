# 3. Ncube high level view

Date: 2020-03-25

## Status

Accepted

## Context

Ncube supports multiple workflows and setups. On the one hand, there is the use
case of an individual that runs Ncube solely on her personal desktop. The other
use case consists of several people working on the same centralized database.
There is further the discovery browser extension that connects to Ncube to store
queries for further preservation.

## Decision

Ncube consist of three high-level components:

- `ncubed` :: A daemon process that runs in the background and provides the core
  functionality of Ncube. It provides two TCP ports, one that provides and API
  for administrative tasks, and the other that gives access to the collections
  and acts as an interface for all user-oriented tasks.
- `ncube` :: The actual UI of Ncube is a separate application. When started, it
  automatically starts `ncubed` if necessary. It communicates with `ncubed` over
  a TCP socket.
- `ncubectl` :: A CLI application that provides commands for administrative
  tasks. It connects to `ncubed` over the second TCP port.

## Consequences

The above model sounds very much like a typical Unix application. The naming and
description even shares common Unix terminology. It is unclear to me how to make
this work on Windows. There is the
[`windows-service-rs`](https://github.com/mullvad/windows-service-rs) crate that
allows to run windows services, which I believe are the Windows equivalent of
daemon processes on Unix.

The development of Ncube happens in a Cargo workspace and is broken into at
least three separate packages, one for each high level components.
