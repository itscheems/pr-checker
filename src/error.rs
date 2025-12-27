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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Configuration error: {0}")]
	Config(String),

	#[error("GitHub API error: {0}")]
	GitHubApi(String),

	#[error("GitHub event parsing error: {0}")]
	EventParse(String),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("YAML parsing error: {0}")]
	Yaml(#[from] serde_yaml::Error),

	#[error("JSON parsing error: {0}")]
	Json(#[from] serde_json::Error),

	#[error("HTTP error: {0}")]
	Http(#[from] reqwest::Error),

	#[error("Regex error: {0}")]
	Regex(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
