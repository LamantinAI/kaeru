//! Human-readable result of a committed hygiene pass.

/// The outcome of one pass.
#[derive(Debug, Clone, Default)]
pub struct HygieneReport {
    pub initiative: String,
    pub trigger: String,
    pub archived: usize,
    pub demoted: usize,
    pub promoted: usize,
    /// Candidates dropped because the node changed between collection and
    /// application — the agent's own decision took precedence.
    pub skipped: usize,
    pub core_before: usize,
    pub core_after: usize,
    /// The ceiling `core_after` is measured against, so the summary can say
    /// when the layer is still over it and why the pass could not help.
    pub core_ceiling: usize,
    /// One line per applied move, for the durable episode.
    pub lines: Vec<String>,
    /// The pass was asked to stop at a batch boundary (daemon shutting down).
    /// Not an error: the next pass re-derives whatever is left.
    pub stopped_early: bool,
}

impl HygieneReport {
    /// Total moves applied.
    pub fn applied(&self) -> usize {
        self.archived + self.demoted + self.promoted
    }

    /// The one-line cue that rides along on the next tool response.
    ///
    /// Deliberately **not** a work report. The agent never asked for the pass
    /// and doesn't know the nodes it touched, so a tally of
    /// archived/demoted/promoted is accounting it can't act on. What it needs
    /// to know is that the ground moved under an open session — and what to do
    /// about it. Same `↳` shape as the other in-result hints: state the
    /// consequence, point at the next step, keep the detail one call away
    /// (`hygiene <initiative>`, plus the durable episode).
    pub fn headline(&self) -> String {
        let n = self.applied();
        let nodes = if n == 1 { "node" } else { "nodes" };
        let init = &self.initiative;
        format!(
            "↳ memory shifted under you — {n} {nodes} re-layered in `{init}`.\n  \
             Re-run `awake {init}` if you're mid-session; `hygiene {init}` for what moved."
        )
    }

    /// The tally, for whoever **asked** for the pass (`hygiene force=true`).
    ///
    /// The counterpart to [`headline`](Self::headline): same pass, different
    /// reader. Someone who ran it on purpose wants to know what it did; an
    /// agent who didn't ask wants to know only that the ground moved.
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if self.archived > 0 {
            parts.push(format!("{} archived", self.archived));
        }
        if self.demoted > 0 {
            parts.push(format!("core {} → {}", self.core_before, self.core_after));
        }
        if self.promoted > 0 {
            parts.push(format!("{} promoted", self.promoted));
        }
        if self.skipped > 0 {
            parts.push(format!("{} skipped (touched meanwhile)", self.skipped));
        }
        if parts.is_empty() {
            parts.push("nothing to do".to_string());
        }
        let mut line = format!(
            "[hygiene {}] {} · trigger: {}",
            self.initiative,
            parts.join(" · "),
            self.trigger
        );
        // A standing condition the pass could not clear must be said out loud.
        // "nothing to do" beside an oversized core reads as "core is fine",
        // and the only thing that gets it down is a hand the reader doesn't
        // know to lift (#75). Every node the pass may move it has already
        // moved by this point, so what is left is pinned — name that, and the
        // one verb that undoes it.
        if self.core_ceiling > 0 && self.core_after > self.core_ceiling {
            let over = self.core_after - self.core_ceiling;
            line.push_str(&format!(
                "\n  ↳ core still holds {} (ceiling {}); the {over} over are pinned, \
                 and a pin outranks the sweep. `unpin <name>` to release one, \
                 or `layer <name> warm` to move it yourself.",
                self.core_after, self.core_ceiling
            ));
        }
        line
    }
}
