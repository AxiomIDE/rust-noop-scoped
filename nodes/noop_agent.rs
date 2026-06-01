use crate::axiom_context::AxiomContext;
use crate::gen::messages::{ NoopInput, NoopOutput };

/// TODO: replace with your description — this doc comment is extracted at
/// publish time and shown in the Axiom registry as this node's documentation.
/// Example: "Validates an incoming order request and returns a confirmation
/// with a generated order ID and calculated total."
///
/// `ax` is the AxiomContext (ADR-001): platform capabilities are reached
/// through it — `ax.log()`, `ax.secrets()`, `ax.agent()`, `ax.reflection()`,
/// `ax.mutation()`. Node code never talks to the platform directly.
pub fn noop_agent(
    ax: &dyn AxiomContext,
    input: NoopInput,
) -> Result<NoopOutput, Box<dyn std::error::Error>> {
    // Exercise the ADR-001 AxiomContext surface directly from node code.
    let mut attrs = std::collections::HashMap::new();
    attrs.insert("text", input.example_string.clone());
    ax.log().info("noop_agent handling", &attrs);

    // Reflection (ADR-050/055): observe the running graph from inside the node.
    let node_count = ax.reflection().flow().nodes().len() as i32;

    // Echo the input through, annotating the int with the reflected node count
    // so the transformation is observable end-to-end.
    Ok(NoopOutput {
        example_string: input.example_string,
        example_int: input.example_int + node_count,
    })
}
