use anyhow::Result;
use clap::{Args, ValueEnum};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum RulesScope {
    Local,
    Semantic,
    All,
}

impl Display for RulesScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RulesScope::Local => write!(f, "local"),
            RulesScope::Semantic => write!(f, "semantic"),
            RulesScope::All => write!(f, "all"),
        }
    }
}

#[derive(Args, Debug)]
pub struct RulesArgs {
    /// Output as JSON instead of human-readable text
    #[arg(long)]
    pub json: bool,

    /// Scope to list (local rulesets, semantic rulesets, or all)
    #[arg(long, default_value_t = RulesScope::All)]
    pub scope: RulesScope,
}

#[derive(Serialize)]
struct RuleInfo<'a> {
    id: &'a str,
    category: &'a str,
}

#[derive(Serialize)]
struct RuleSetInfo<'a> {
    id: &'a str,
    rules: Vec<RuleInfo<'a>>,
}

pub fn execute(args: RulesArgs) -> Result<()> {
    use bsharp_analysis::framework::registry::AnalyzerRegistry;
    use bsharp_analysis::framework::Rule;

    let registry = AnalyzerRegistry::default_registry();

    let mut sets: Vec<RuleSetInfo> = Vec::new();

    let include_local = matches!(args.scope, RulesScope::Local | RulesScope::All);
    let include_semantic = matches!(args.scope, RulesScope::Semantic | RulesScope::All);

    if include_local {
        for rs in registry.rulesets_local() {
            let mut rules: Vec<RuleInfo> = Vec::new();
            for r in rs.iter() {
                rules.push(RuleInfo {
                    id: r.id(),
                    category: r.category(),
                });
            }
            // stable order by rule id
            rules.sort_by(|a, b| a.id.cmp(b.id));
            sets.push(RuleSetInfo { id: rs.id, rules });
        }
    }

    if include_semantic {
        for rs in registry.rulesets_semantic() {
            let mut rules: Vec<RuleInfo> = Vec::new();
            for r in rs.iter() {
                rules.push(RuleInfo {
                    id: r.id(),
                    category: r.category(),
                });
            }
            rules.sort_by(|a, b| a.id.cmp(b.id));
            sets.push(RuleSetInfo { id: rs.id, rules });
        }
    }

    // stable order by ruleset id
    sets.sort_by(|a, b| a.id.cmp(b.id));

    if args.json {
        println!("{}", serde_json::to_string_pretty(&sets)?);
        return Ok(());
    }

    // human readable
    for rs in &sets {
        println!("Ruleset: {}", rs.id);
        for r in &rs.rules {
            println!("  - {} [{}]", r.id, r.category);
        }
    }

    Ok(())
}
