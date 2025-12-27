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

use crate::config::TitleRule;
use crate::github::PullRequest;
use crate::rules::{RuleResult, Violation};
use regex::Regex;

pub fn check_title(pr: &PullRequest, rule: &TitleRule) -> RuleResult {
	let mut violations = Vec::new();

	// Check pattern
	if let Some(pattern) = &rule.pattern {
		match Regex::new(pattern) {
			Ok(re) => {
				if !re.is_match(&pr.title) {
					violations.push(Violation {
						message: format!(
							"PR title '{}' does not match required pattern: {}",
							pr.title, pattern
						),
					});
				}
			}
			Err(e) => {
				violations.push(Violation {
					message: format!("Invalid regex pattern '{}': {}", pattern, e),
				});
			}
		}
	}

	// Check min_length
	if let Some(min_len) = rule.min_length
		&& pr.title.len() < min_len
	{
		violations.push(Violation {
			message: format!(
				"PR title '{}' is too short ({} chars), minimum required: {}",
				pr.title,
				pr.title.len(),
				min_len
			),
		});
	}

	// Check max_length
	if let Some(max_len) = rule.max_length
		&& pr.title.len() > max_len
	{
		violations.push(Violation {
			message: format!(
				"PR title '{}' is too long ({} chars), maximum allowed: {}",
				pr.title,
				pr.title.len(),
				max_len
			),
		});
	}

	violations
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_pr(title: &str) -> PullRequest {
		PullRequest {
			number: 1,
			title: title.to_string(),
			labels: vec![],
		}
	}

	#[test]
	fn test_pattern_match() {
		let pr = create_pr("feat: add new feature");
		let rule = TitleRule {
			pattern: Some("^(feat|fix|docs):".to_string()),
			min_length: None,
			max_length: None,
		};

		let violations = check_title(&pr, &rule);
		assert!(violations.is_empty());
	}

	#[test]
	fn test_pattern_mismatch() {
		let pr = create_pr("invalid title");
		let rule = TitleRule {
			pattern: Some("^(feat|fix|docs):".to_string()),
			min_length: None,
			max_length: None,
		};

		let violations = check_title(&pr, &rule);
		assert_eq!(violations.len(), 1);
		assert!(violations[0].message.contains("does not match"));
	}

	#[test]
	fn test_min_length() {
		let pr = create_pr("short");
		let rule = TitleRule {
			pattern: None,
			min_length: Some(10),
			max_length: None,
		};

		let violations = check_title(&pr, &rule);
		assert_eq!(violations.len(), 1);
		assert!(violations[0].message.contains("too short"));
	}

	#[test]
	fn test_max_length() {
		let pr = create_pr("a".repeat(100).as_str());
		let rule = TitleRule {
			pattern: None,
			min_length: None,
			max_length: Some(50),
		};

		let violations = check_title(&pr, &rule);
		assert_eq!(violations.len(), 1);
		assert!(violations[0].message.contains("too long"));
	}
}
