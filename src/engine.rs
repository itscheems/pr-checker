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

		Ok(all_violations)
	}
}
