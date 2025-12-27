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

use crate::config::Config;
use crate::github::GitHubClient;
use crate::rules::Violation;
use crate::rules::{RuleResult, check_labels, check_title};

pub struct Engine {
	client: GitHubClient,
	config: Config,
}

impl Engine {
	pub fn new(client: GitHubClient, config: Config) -> Self {
		Self { client, config }
	}

	pub async fn run(&self, pr_number: u64) -> crate::error::Result<RuleResult> {
		let pr = self.client.get_pr(pr_number).await?;

		let mut all_violations = Vec::new();

		// Check title rule
		if let Some(title_rule) = &self.config.title {
			let violations = check_title(&pr, title_rule);
			all_violations.extend(violations);
		}

		// Check labels rule
		if let Some(labels_rule) = &self.config.labels {
			let violations = check_labels(&pr, labels_rule);
			all_violations.extend(violations);
		}

		// Ensure title type aligns with kind/* label
		// Only check if required labels are configured and non-empty
		if let Some(labels_rule) = &self.config.labels
			&& let Some(required) = &labels_rule.required
			&& !required.is_empty()
		{
			// Check if title type's expected label is in required list
			if let Some(expected) = expected_label_for_title(&pr.title)
				&& required.contains(&expected.to_string())
				&& !has_label(&pr, expected)
			{
				all_violations.push(Violation {
					message: format!(
						"Title type '{}' requires label '{}', current labels: [{}], title: '{}'",
						title_type(&pr.title),
						expected,
						format_labels(&pr),
						pr.title
					),
				});
			}
		}

		// Prepend a context line with title and labels if there are violations
		if !all_violations.is_empty() {
			all_violations.insert(
				0,
				Violation {
					message: format!(
						"Context -> title: '{}'; labels: [{}]",
						pr.title,
						format_labels(&pr)
					),
				},
			);
		}

		Ok(all_violations)
	}
}

fn has_label(pr: &crate::github::PullRequest, name: &str) -> bool {
	pr.labels.iter().any(|l| l.name == name)
}

fn format_labels(pr: &crate::github::PullRequest) -> String {
	if pr.labels.is_empty() {
		"none".to_string()
	} else {
		pr.labels
			.iter()
			.map(|l| l.name.clone())
			.collect::<Vec<_>>()
			.join(", ")
	}
}

fn title_type(title: &str) -> String {
	let prefix = title.split(':').next().unwrap_or_default().trim();
	// Support optional component scope, e.g., feat(api-server): ...
	let type_only = prefix.split('(').next().unwrap_or(prefix).trim();
	type_only.to_lowercase()
}

fn expected_label_for_title(title: &str) -> Option<&'static str> {
	match title_type(title).as_str() {
		"feat" => Some("kind/feature"),
		"fix" => Some("kind/bug"),
		"docs" => Some("kind/docs"),
		"chore" => Some("kind/chore"),
		"refactor" => Some("kind/refactor"),
		"test" => Some("kind/test"),
		"perf" => Some("kind/performance"),
		"ci" => Some("kind/ci"),
		"build" => Some("kind/build"),
		"security" => Some("kind/security"),
		"dependencies" => Some("kind/dependencies"),
		_ => None,
	}
}
