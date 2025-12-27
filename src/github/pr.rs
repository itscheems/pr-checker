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

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequest {
	pub number: u64,
	pub title: String,
	pub labels: Vec<PullRequestLabel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PullRequestLabel {
	pub name: String,
}

#[derive(Debug, Deserialize)]
struct GitHubEvent {
	#[serde(rename = "pull_request")]
	pull_request: Option<PullRequest>,
	repository: Option<Repository>,
}

#[derive(Debug, Deserialize)]
struct Repository {
	owner: Owner,
	name: String,
}

#[derive(Debug, Deserialize)]
struct Owner {
	login: String,
}

pub struct GitHubClient {
	client: reqwest::Client,
	token: String,
	owner: String,
	repo: String,
}

impl GitHubClient {
	pub fn new(token: String, owner: String, repo: String) -> Self {
		Self {
			client: reqwest::Client::new(),
			token,
			owner,
			repo,
		}
	}

	pub fn from_env() -> crate::error::Result<Self> {
		let token = std::env::var("GITHUB_TOKEN")
			.map_err(|_| crate::error::Error::Config("GITHUB_TOKEN not set".to_string()))?;

		let event_path = std::env::var("GITHUB_EVENT_PATH")
			.map_err(|_| crate::error::Error::Config("GITHUB_EVENT_PATH not set".to_string()))?;

		let event_content = std::fs::read_to_string(&event_path)?;
		let event: GitHubEvent = serde_json::from_str(&event_content)?;

		let (owner, repo) = if let Some(repo_info) = event.repository {
			(repo_info.owner.login, repo_info.name)
		} else {
			return Err(crate::error::Error::EventParse(
				"Repository information not found in event".to_string(),
			));
		};

		Ok(Self::new(token, owner, repo))
	}

	pub async fn get_pr(&self, pr_number: u64) -> crate::error::Result<PullRequest> {
		let url = format!(
			"https://api.github.com/repos/{}/{}/pulls/{}",
			self.owner, self.repo, pr_number
		);

		let response = self
			.client
			.get(&url)
			.header("Authorization", format!("Bearer {}", self.token))
			.header("Accept", "application/vnd.github.v3+json")
			.header("User-Agent", "pr-checker")
			.send()
			.await?;

		if !response.status().is_success() {
			return Err(crate::error::Error::GitHubApi(format!(
				"Failed to fetch PR: {}",
				response.status()
			)));
		}

		let pr: PullRequest = response.json().await?;
		Ok(pr)
	}

	pub fn parse_pr_number_from_event() -> crate::error::Result<u64> {
		let event_path = std::env::var("GITHUB_EVENT_PATH")
			.map_err(|_| crate::error::Error::Config("GITHUB_EVENT_PATH not set".to_string()))?;

		let event_content = std::fs::read_to_string(&event_path)?;
		let event: GitHubEvent = serde_json::from_str(&event_content)?;

		if let Some(pr) = event.pull_request {
			Ok(pr.number)
		} else {
			Err(crate::error::Error::EventParse(
				"Pull request not found in event".to_string(),
			))
		}
	}
}
