use std::{fs, io, path::Path};
use anyhow::Result;
use async_trait::async_trait;
use tracing::info;
use crate::{TestCaseGenerator, NamedTestCase, TestCase, TestCaseError};

/// Generator for test cases from JSON files.
pub struct JsonTestCaseGenerator {
    /// Directory path containing JSON test files.
    pub directory_path: String,
}

impl JsonTestCaseGenerator {
    /// Creates a new JSON test case generator.
    pub fn new(directory_path: String) -> Self {
        Self { directory_path }
    }

    /// Recursively finds all JSON files in a directory.
    fn find_json_files(&self, dir: &Path) -> Result<Vec<std::path::PathBuf>, TestCaseError> {
        let mut json_files = Vec::new();
        
        for entry in fs::read_dir(dir).map_err(TestCaseError::Io)? {
            let entry = entry.map_err(TestCaseError::Io)?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively search subdirectories
                let sub_files = self.find_json_files(&path)?;
                json_files.extend(sub_files);
            } else if path.is_file() {
                // Check if it's a JSON file
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        json_files.push(path);
                    }
                }
            }
        }
        
        Ok(json_files)
    }

    /// Processes a single JSON file to extract test cases.
    fn process_json_file(&self, json_file: &Path) -> Result<Vec<NamedTestCase>, TestCaseError> {
        // Read the JSON file
        let json_content = fs::read_to_string(json_file)
            .map_err(|e| TestCaseError::Io(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to read file {}: {}", json_file.display(), e)
            )))?;
        
        // Parse the JSON file
        let test_cases: std::collections::HashMap<String, TestCase> = serde_json::from_str(&json_content)
            .map_err(|e| TestCaseError::Serde(e))?;
        
        info!("Found {} test cases in {}", test_cases.len(), json_file.display());
        
        // Convert to NamedTestCase format
        let named_test_cases: Vec<NamedTestCase> = test_cases
            .into_iter()
            .map(|(name, test_case)| NamedTestCase { name, test_case })
            .collect();
        
        Ok(named_test_cases)
    }
}

#[async_trait]
impl TestCaseGenerator for JsonTestCaseGenerator {
    async fn generate(&self) -> Result<Vec<NamedTestCase>> {
        let dir_path = Path::new(&self.directory_path);
        
        if !dir_path.exists() {
            return Err(anyhow::anyhow!("Directory {} does not exist", self.directory_path));
        }
        
        if !dir_path.is_dir() {
            return Err(anyhow::anyhow!("{} is not a directory", self.directory_path));
        }
        
        info!("Scanning directory: {}", self.directory_path);
        
        // Collect all JSON files recursively
        let json_files = self.find_json_files(dir_path)?;
        
        if json_files.is_empty() {
            info!("No JSON files found in directory: {}", self.directory_path);
            return Ok(Vec::new());
        }
        
        info!("Found {} JSON files to process", json_files.len());
        
        let mut all_test_cases = Vec::new();
        
        // Process each JSON file
        for json_file in json_files {
            info!("Processing file: {}", json_file.display());
            
            match self.process_json_file(&json_file) {
                Ok(test_cases) => {
                    let count = test_cases.len();
                    all_test_cases.extend(test_cases);
                    info!("Successfully processed {} test cases from {}", 
                          count, json_file.display());
                }
                Err(e) => {
                    eprintln!("Error processing {}: {}", json_file.display(), e);
                }
            }
        }
        
        info!("Total test cases generated: {}", all_test_cases.len());
        Ok(all_test_cases)
    }

    async fn generate_to_path(&self, path: &Path) -> Result<usize> {
        let test_cases = self.generate().await?;
        
        if test_cases.is_empty() {
            return Ok(0);
        }
        
        // Group test cases by file name for better organization
        let mut grouped_cases: std::collections::HashMap<String, Vec<NamedTestCase>> = 
            std::collections::HashMap::new();
        
        for test_case in test_cases {
            let file_name = format!("{}.json", test_case.name);
            grouped_cases.entry(file_name).or_default().push(test_case);
        }
        
        let mut total_written = 0;
        
        for (file_name, cases) in grouped_cases {
            let file_path = path.join(&file_name);
            NamedTestCase::to_path(&file_path, &cases)?;
            total_written += cases.len();
            info!("Written {} test cases to {}", cases.len(), file_path.display());
        }
        
        Ok(total_written)
    }
} 