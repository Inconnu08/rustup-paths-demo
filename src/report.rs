use crate::migrate::MigrationPlan;
use crate::resolver::ResolutionReport;

pub fn print_resolve(report: &ResolutionReport, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(&report.resolved).unwrap());
        return;
    }

    println!("config_dir: {}", report.resolved.config_dir.display());
    println!("data_dir:   {}", report.resolved.data_dir.display());
    println!("cache_dir:  {}", report.resolved.cache_dir.display());
    println!("bin_dir:    {}", report.resolved.bin_dir.display());
}

pub fn print_explain(report: &ResolutionReport, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(report).unwrap());
        return;
    }

    if !report.warnings.is_empty() {
        println!("Warnings:");
        for w in &report.warnings {
            println!("- {w}");
        }
        println!();
    }

    for decision in &report.decisions {
        println!("{}: {}", decision.name, decision.selected_path.display());
        println!("  reason: {}", decision.reason);
        println!();
    }
}

pub fn print_migration_plan(plan: &MigrationPlan, json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(plan).unwrap());
        return;
    }

    if !plan.warnings.is_empty() {
        println!("Warnings:");
        for w in &plan.warnings {
            println!("- {w}");
        }
        println!();
    }

    for step in &plan.steps {
        println!("- {} -> {}", step.from.display(), step.to.display());
        println!("  {}", step.description);
        println!();
    }
}