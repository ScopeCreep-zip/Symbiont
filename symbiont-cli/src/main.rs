//! Symbiont CLI - Run simulations from the command line.

use clap::{Parser, Subcommand, ValueEnum};
use std::io::Write;
use symbiont_core::capability::common;
use symbiont_sim::network::NetworkConfig;
use symbiont_sim::runner::{quick_run, SimulationConfig, SimulationRunner};
use symbiont_sim::scenarios::{AdversaryScenario, TrustEmergenceScenario, WorkflowScenario};
use symbiont_sim::scenarios::adversary::AdversaryType;
use symbiont_sim::scenarios::workflow::WorkflowType;
use symbiont_sim::scenarios::Scenario;

#[derive(Parser)]
#[command(name = "symbiont")]
#[command(about = "Symbiont Protocol Simulator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a simulation scenario
    Run {
        /// Scenario to run
        #[arg(short, long, value_enum)]
        scenario: ScenarioArg,

        /// Number of nodes
        #[arg(short, long, default_value = "20")]
        nodes: usize,

        /// Number of ticks to simulate
        #[arg(short, long, default_value = "500")]
        ticks: u64,

        /// Random seed for reproducibility
        #[arg(long)]
        seed: Option<u64>,

        /// Connection probability
        #[arg(long, default_value = "0.3")]
        connection_prob: f64,

        /// Export trust history to CSV file
        #[arg(long)]
        export_trust: Option<String>,

        /// For adversary scenarios: when to inject
        #[arg(long, default_value = "50")]
        inject_at: u64,

        /// For adversary scenarios: number of adversaries
        #[arg(long, default_value = "3")]
        adversary_count: usize,

        /// For strategic adversaries: when to defect
        #[arg(long, default_value = "200")]
        defect_at: u64,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Quick simulation with defaults
    Quick {
        /// Number of nodes
        #[arg(short, long, default_value = "10")]
        nodes: usize,

        /// Number of ticks
        #[arg(short, long, default_value = "100")]
        ticks: u64,
    },

    /// Show information about scenarios
    Info {
        /// Scenario to describe
        #[arg(value_enum)]
        scenario: Option<ScenarioArg>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum ScenarioArg {
    /// Trust emergence in honest network
    TrustEmergence,
    /// Strategic adversary injection
    Strategic,
    /// Free rider injection
    FreeRider,
    /// Sybil cluster injection
    Sybil,
    /// Chain workflow routing
    WorkflowChain,
    /// Fan-out/fan-in workflow
    WorkflowFanOut,
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            scenario,
            nodes,
            ticks,
            seed,
            connection_prob,
            export_trust,
            inject_at,
            adversary_count,
            defect_at,
            verbose,
        } => {
            run_simulation(
                scenario,
                nodes,
                ticks,
                seed,
                connection_prob,
                export_trust,
                inject_at,
                adversary_count,
                defect_at,
                verbose,
            );
        }

        Commands::Quick { nodes, ticks } => {
            println!("Running quick simulation: {nodes} nodes, {ticks} ticks");
            let result = quick_run(nodes, ticks);
            println!("\n{}", result.summary);
            println!("Completed in {}ms", result.duration_ms);
        }

        Commands::Info { scenario } => {
            show_info(scenario);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_simulation(
    scenario_arg: ScenarioArg,
    nodes: usize,
    ticks: u64,
    seed: Option<u64>,
    connection_prob: f64,
    export_trust: Option<String>,
    inject_at: u64,
    adversary_count: usize,
    defect_at: u64,
    verbose: bool,
) {
    // Build network config
    let mut network_config = NetworkConfig::default()
        .with_nodes(nodes)
        .with_capability(common::analysis())
        .with_capability(common::generation())
        .with_capability(common::transformation())
        .with_capability(common::validation())
        .with_connection_prob(connection_prob);

    if let Some(s) = seed {
        network_config = network_config.with_seed(s);
    }

    // Build simulation config
    let sim_config = SimulationConfig::default()
        .with_ticks(ticks)
        .with_network(network_config);

    // Create scenario
    let scenario: Box<dyn Scenario> = match scenario_arg {
        ScenarioArg::TrustEmergence => {
            Box::new(TrustEmergenceScenario::new())
        }
        ScenarioArg::Strategic => {
            Box::new(
                AdversaryScenario::new(AdversaryType::Strategic)
                    .inject_at(inject_at)
                    .with_count(adversary_count)
                    .defect_at(defect_at),
            )
        }
        ScenarioArg::FreeRider => {
            Box::new(
                AdversaryScenario::new(AdversaryType::FreeRider)
                    .inject_at(inject_at)
                    .with_count(adversary_count),
            )
        }
        ScenarioArg::Sybil => {
            Box::new(
                AdversaryScenario::new(AdversaryType::Sybil)
                    .inject_at(inject_at)
                    .with_count(adversary_count),
            )
        }
        ScenarioArg::WorkflowChain => {
            Box::new(WorkflowScenario::new(WorkflowType::Chain))
        }
        ScenarioArg::WorkflowFanOut => {
            Box::new(WorkflowScenario::new(WorkflowType::FanOutFanIn))
        }
    };

    println!("Running scenario: {}", scenario.name());
    println!("  Description: {}", scenario.description());
    println!("  Nodes: {nodes}, Ticks: {ticks}");
    println!();

    // Run simulation with progress
    let mut runner = SimulationRunner::new(sim_config);
    runner.apply_scenario(scenario.as_ref());

    let (result, runner) = if verbose {
        // Add progress callback
        let mut runner = runner.on_progress(Box::new(move |current, total| {
            print!("\rProgress: {}/{} ({:.1}%)", current, total, (current as f64 / total as f64) * 100.0);
            std::io::stdout().flush().unwrap();
        }));
        let result = runner.run();
        println!(); // New line after progress
        (result, runner)
    } else {
        let result = runner.run();
        (result, runner)
    };

    print_result(&result, &runner);

    // Export if requested
    if let Some(path) = export_trust {
        let mut file = std::fs::File::create(&path).expect("Failed to create export file");
        runner.network().metrics.export_trust_csv(&mut file).expect("Failed to write CSV");
        println!("\nExported trust history to: {path}");
    }
}

fn print_result(result: &symbiont_sim::runner::SimulationResult, runner: &SimulationRunner) {
    println!("\n{}", result.summary);
    println!("Duration: {}ms", result.duration_ms);

    // Network stats
    let stats = runner.network().stats();
    println!("\nNetwork State:");
    println!("  Nodes: {}", stats.node_count);
    println!("  Connections: {}", stats.connection_count);
    println!("  Mean Trust: {:.3}", stats.mean_trust);
    println!("  Mean Connections/Node: {:.1}", stats.mean_connections);
}

fn show_info(scenario: Option<ScenarioArg>) {
    match scenario {
        Some(s) => {
            let (name, desc, details) = scenario_details(s);
            println!("Scenario: {name}");
            println!("Description: {desc}");
            println!("\nDetails:");
            println!("{details}");
        }
        None => {
            println!("Available scenarios:\n");
            for s in [
                ScenarioArg::TrustEmergence,
                ScenarioArg::Strategic,
                ScenarioArg::FreeRider,
                ScenarioArg::Sybil,
                ScenarioArg::WorkflowChain,
                ScenarioArg::WorkflowFanOut,
            ] {
                let (name, desc, _) = scenario_details(s);
                println!("  {name:20} - {desc}");
            }
            println!("\nUse 'symbiont info <scenario>' for more details.");
        }
    }
}

fn scenario_details(scenario: ScenarioArg) -> (&'static str, &'static str, &'static str) {
    match scenario {
        ScenarioArg::TrustEmergence => (
            "trust-emergence",
            "Watch trust dynamics emerge in an honest network",
            "All nodes behave honestly with configurable quality.\n\
             Observe how trust scores converge over time.\n\
             Expected outcome: stable, high trust distribution.",
        ),
        ScenarioArg::Strategic => (
            "strategic",
            "Strategic adversary injection",
            "Adversaries behave well initially to build trust,\n\
             then defect at a specified tick.\n\
             Tests the protocol's ability to detect reputation manipulation.\n\
             Options: --inject-at, --adversary-count, --defect-at",
        ),
        ScenarioArg::FreeRider => (
            "free-rider",
            "Free rider injection",
            "Agents that receive value but provide poor quality.\n\
             Tests reciprocity dynamics and quality-based filtering.\n\
             Options: --inject-at, --adversary-count",
        ),
        ScenarioArg::Sybil => (
            "sybil",
            "Sybil cluster injection",
            "Coordinated fake identities that rate each other highly.\n\
             Tests diversity requirements and collusion detection.\n\
             Options: --inject-at, --adversary-count",
        ),
        ScenarioArg::WorkflowChain => (
            "workflow-chain",
            "Chain workflow routing test",
            "Sequential workflow: A -> B -> C\n\
             Tests task routing based on capability and trust.\n\
             Measures routing quality and completion rate.",
        ),
        ScenarioArg::WorkflowFanOut => (
            "workflow-fan-out",
            "Fan-out/fan-in workflow routing test",
            "Parallel workflow with merge step.\n\
             Tests load balancing and ensemble aggregation.\n\
             Measures parallel efficiency and result quality.",
        ),
    }
}
