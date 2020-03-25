# 2. Ncube - Purposes and Goals

Date: 2020-03-25

## Status

Accepted

## Context

[Sugarcube](https://github.com/critocrito/sugarcube) is a framework to define
data processes with the goal to preserve data. Its aimed at human rights
investigators that want to use publicly available data as a tool for their
investigations. Sugarcube is used as a preservation framework for [Syrian
Archive](https://syrianarchive.org/en).

While Sugarcube makes it easy to collect vast and diverse data, exploring that
data and finding interesting bits of data is quite difficult. It requires deep
insights into database technologies and export to external formats, e.g. CSV
and spreadsheets.

There is a variety of tooling available for human rights investigators. Most
tooling is developed as a web based client/server application. This has a few
tradeoffs that are commonly not addressed:

- Data is stored on a remote server and issues around security, access and
  ownership of the data are not clear.
- Web based client/server application models tend to impose a high level of
  operational cost in the case investigators want to self host.
- These tools require an always-on internet connection and are generally
  developed based on US/European expectations about such internet connection.
- Most tools are developed with an abundance of computational ressources
  available. But many investigators don't have such ressources available or
  don't require such a scale for their needs. 
  
## Decision

Ncube is a graphical interface that allows human rights investigators to explore
and verify data that was collected by Sugarcube. It aims to be a basic tool for
investigators and their data investigation needs. On a high level it guides
investigators to produce a set of verified data records, about which a statement
of truth can be made.

To address the above mentioned tradeoffs, Ncube is designed to be a
**local-first** application. It is distributed as a classical desktop
application for Linux, macOS and Windows. It features two modes of operation:

- The default mode is to operate local only by collecting data on the desktop
  computer itself. It downloads files locally and stores data in a local SQLite
  database.
- To allow collaboration and to scale computational demands Ncube can connect to
  remote instances of Ncuube to operate on a central database.
  
There is a third mode imagineable that would allow to collaborate in a decentral
manner. This could be achieved using technologies like IPFS and CRDT's but are
out of scope for the current project frame but would be very interesting in the
future.

Sugarcube is developed in Javascript and superficially it would make sense to
develop Ncube as an Electron app to reduce the amount of friction. But from
experience I very much dislike the performance characteristics of Electron apps.
To be as performant as possible I choose **Rust** as primary implementation
language. I expect that this allows me to develop a memory efficient version of
Ncube that makes use of all the cores available on a machine. Since Rust
compiles down to a single binary, distribution will be simpler. Rust also allows
to develop cross platform code that runs natively on all supported platforms.
But of course, I feel like learning something new. 

The default use case for a project is to run on a desktop computer. Files are
stored locally on the hard disk and data is stored in a local SQLite database.
Users can evolve the use of Ncube based on their needs by falling back on
Sugarcube. Compatibility between Sugarcube and Ncube has to be maintained
therefore.

## Consequences

The choice of Rust as implementation language has a few radical consequences:

- If Ncube wants to support to run data preservation tasks from within it needs
  to bridge into the Javascript world. Either by calling the NodeJS interpreter
  as a sub process or by embedding V8 bindings into Rust. The former seems
  simpler to me but I would need to know how this can work on Windows. Ncube has
  to be distributed with all dependencies,
  [node-packer](https://github.com/pmq20/node-packer) as an interesting approach.
- There is no widely accepted approach to cross platform GUI development, but
  there are [many](https://github.com/xi-editor/druid)
  [interesting](https://github.com/maps4print/azul)
  [approaches](https://areweguiyet.com/#ecosystem). I don't have much experience
  with native GUI development anyway. But I do have experience with developing
  web interfaces using React. I stumbled across two approaches to embed a
  browser window in a Rust application,
  [web-view](https://github.com/Boscop/web-view) and
  [tauri](https://github.com/tauri-apps/tauri). The latter aims to be a more
  complete framework, it provides an API to the browser frontend based on
  message passing. The former is very bare bones and provides just an embeddable
  browser window. Any interaction with the native Rust backend would need to be
  added.
