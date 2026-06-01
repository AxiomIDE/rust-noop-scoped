use crate::axiom_context::AxiomContext;
use crate::gen::messages::{ NoopInput, NoopOutput };

/// Memory probe node — exercises the full ADR-001 AxiomContext surface, with an
/// emphasis on the live ADR-002 memory hierarchy. It appends a turn to a
/// session, reads the history back, and runs a semantic search, then encodes
/// the round-trip result into its output. This proves the Rust SDK's
/// async→sync memory bridge (block_in_place over the sidecar MemoryProxyService)
/// works at runtime, not just at compile time.
///
/// `ax` is the AxiomContext (ADR-001): platform capabilities are reached
/// through it — `ax.log()`, `ax.secrets()`, `ax.agent()`, `ax.reflection()`,
/// `ax.mutation()`. Node code never talks to the platform directly.
pub fn noop_agent(
    ax: &dyn AxiomContext,
    input: NoopInput,
) -> Result<NoopOutput, Box<dyn std::error::Error>> {
    let mut attrs = std::collections::HashMap::new();
    attrs.insert("text", input.example_string.clone());
    ax.log().info("noop_agent handling", &attrs);

    // Reflection (ADR-050/055): observe the running graph from inside the node.
    let node_count = ax.reflection().flow().nodes().len() as i32;

    // ── Live memory round-trip (ADR-002) ────────────────────────────────────
    // The session id comes from the typed input (never inferred). Append a
    // turn, read the history back, and run a semantic search — each call
    // crosses the async→sync bridge into the sidecar memory proxy. Errors
    // propagate as a node failure so a broken bridge is loudly visible.
    let session_id = if input.example_string.is_empty() {
        "rust-mem-probe".to_string()
    } else {
        input.example_string.clone()
    };
    let mem = ax.agent().memory();
    let session = mem.session(&session_id);
    session
        .history()
        .append("user", &format!("probe input={}", input.example_int))?;
    let turns = session.history().last(10)?;
    let hits = session.search("probe", 5)?;

    // Encode the round-trip outcome so it is observable end-to-end: the string
    // reports the turn/search counts, the int folds in the reflected node count.
    Ok(NoopOutput {
        example_string: format!("mem-ok turns={} hits={}", turns.len(), hits.len()),
        example_int: input.example_int + node_count,
    })
}
