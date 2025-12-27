// Copyright 2025 chenjjiaa
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod config;
mod engine;
mod error;
mod github;
mod rules;

use clap::Parser;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "pr-checker")]
#[command(about = "Validate pull request rules")]
struct Args {
	/// Path to config file
	#[arg(long)]
	config: Option<String>,
}

fn print_annotation(level: &str, title: &str, message: &str) {
	// GitHub Actions annotation format
	// ::level title=title::message
	println!(
		"::{} title={}::{}",
		level,
		escape_annotation(title),
		escape_annotation(message)
	);
}

fn escape_annotation(s: &str) -> String {
	s.replace('%', "%25")
		.replace('\r', "%0D")
		.replace('\n', "%0A")
		.replace(':', "%3A")
		.replace(',', "%2C")
}

#[tokio::main]
async fn main() {
	// Initialize tracing
	tracing_subscriber::fmt()
		.with_env_filter(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
		)
		.init();

	let args = Args::parse();

	// Get config path from args or GitHub Actions input or default
	let config_path = args
		.config
		.or_else(|| std::env::var("INPUT_CONFIG").ok())
		.unwrap_or_else(|| ".github/pr-checker.yml".to_string());

	let exit_code = match run(config_path.as_str()).await {
		Ok(violations) => {
			if violations.is_empty() {
				info!("All PR checks passed!");
				0
			} else {
				error!("Found {} violation(s)", violations.len());
				for violation in &violations {
					print_annotation("error", "PR validation failed", &violation.message);
				}
				1
			}
		}
		Err(e) => match e {
			error::Error::Config(msg) => {
				error!("Configuration error: {}", msg);
				print_annotation("error", "Configuration error", &msg);
				2
			}
			error::Error::GitHubApi(msg) => {
				error!("GitHub API error: {}", msg);
				print_annotation("error", "GitHub API error", &msg);
				3
			}
			error::Error::EventParse(msg) => {
				error!("Event parsing error: {}", msg);
				print_annotation("error", "Event parsing error", &msg);
				2
			}
			_ => {
				error!("Internal error: {}", e);
				print_annotation("error", "Internal error", &e.to_string());
				10
			}
		},
	};

	std::process::exit(exit_code);
}

async fn run(config_path: &str) -> error::Result<Vec<rules::Violation>> {
	info!("Starting PR checker...");
	info!("Config path: {}", config_path);

	// Load configuration
	let config = config::Config::from_file(config_path)?;
	info!("Configuration loaded successfully");

	// Initialize GitHub client from environment
	let client = github::GitHubClient::from_env()?;
	info!("GitHub client initialized");

	// Parse PR number from event
	let pr_number = github::GitHubClient::parse_pr_number_from_event()?;
	info!("PR number: {}", pr_number);

	// Create engine and run checks
	let engine = engine::Engine::new(client, config);
	let violations = engine.run(pr_number).await?;

	Ok(violations)
}
