// Copyright (C) Microsoft Corporation. All rights reserved.

//! Definitions for the mesh entrypoint.
//!
//! These are here instead of in `hvlite_entry` to support launching hvlite from
//! a foreign mesh host. The only supported use case is launching hvlite from
//! petri for testing.

use mesh::MeshPayload;
use mesh_worker::WorkerHostRunner;

/// The initial message to send when launching a mesh child process.
#[derive(MeshPayload)]
pub struct MeshHostParams {
    pub runner: WorkerHostRunner,
}