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

use crate::config::LabelRule;
use crate::github::PullRequest;
use crate::rules::{RuleResult, Violation};

pub fn check_labels(pr: &PullRequest, rule: &LabelRule) -> RuleResult {
	let mut violations = Vec::new();

	if let Some(required) = &rule.required {
		let pr_label_names: Vec<String> = pr.labels.iter().map(|l| l.name.clone()).collect();

		for required_label in required {
			if !pr_label_names.contains(required_label) {
				violations.push(Violation {
					message: format!(
						"PR is missing required label: '{}'. Current labels: [{}]",
						required_label,
						if pr_label_names.is_empty() {
							"none".to_string()
						} else {
							pr_label_names.join(", ")
						}
					),
				});
			}
		}
	}

	violations
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::github::pr::PullRequestLabel;

	fn create_pr_with_labels(labels: Vec<&str>) -> PullRequest {
		PullRequest {
			number: 1,
			title: "Test PR".to_string(),
			labels: labels
				.into_iter()
				.map(|name| PullRequestLabel {
					name: name.to_string(),
				})
				.collect(),
		}
	}

	#[test]
	fn test_all_required_labels_present() {
		let pr = create_pr_with_labels(vec!["kind/bug", "priority/high"]);
		let rule = LabelRule {
			required: Some(vec!["kind/bug".to_string(), "priority/high".to_string()]),
		};

		let violations = check_labels(&pr, &rule);
		assert!(violations.is_empty());
	}

	#[test]
	fn test_missing_required_label() {
		let pr = create_pr_with_labels(vec!["kind/bug"]);
		let rule = LabelRule {
			required: Some(vec!["kind/bug".to_string(), "priority/high".to_string()]),
		};

		let violations = check_labels(&pr, &rule);
		assert_eq!(violations.len(), 1);
		assert!(violations[0].message.contains("priority/high"));
	}

	#[test]
	fn test_no_labels() {
		let pr = create_pr_with_labels(vec![]);
		let rule = LabelRule {
			required: Some(vec!["kind/bug".to_string()]),
		};

		let violations = check_labels(&pr, &rule);
		assert_eq!(violations.len(), 1);
		assert!(violations[0].message.contains("none"));
	}

	#[test]
	fn test_no_required_labels() {
		let pr = create_pr_with_labels(vec!["kind/bug"]);
		let rule = LabelRule { required: None };

		let violations = check_labels(&pr, &rule);
		assert!(violations.is_empty());
	}
}
