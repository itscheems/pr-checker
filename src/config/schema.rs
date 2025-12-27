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
pub struct Config {
	pub title: Option<TitleRule>,
	pub labels: Option<LabelRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TitleRule {
	/// Regex pattern to match against PR title
	pub pattern: Option<String>,
	/// Minimum length of the title
	pub min_length: Option<usize>,
	/// Maximum length of the title
	pub max_length: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LabelRule {
	/// List of required labels
	pub required: Option<Vec<String>>,
}

impl Config {
	pub fn from_file(path: &str) -> crate::error::Result<Self> {
		let content = std::fs::read_to_string(path)?;
		let config: Config = serde_yaml::from_str(&content)?;
		Ok(config)
	}
}
