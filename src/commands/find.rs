use std::path::PathBuf;
use std::fs;
use std::error::Error;

pub struct FindConfig {
    pub target: String,
    pub root: PathBuf,
}

impl FindConfig {
    pub fn build(target: &str, root: &str) -> Result<FindConfig, Box<dyn Error>> {
        let target = String::from(target);

        let root = std::path::PathBuf::from(root);

        let md = fs::metadata(&root)?;

        if !md.is_dir() {
            if md.is_file() {
                return Err("expected directory to begin search but found file".into())
            } else {
                return Err("expected directory to begin search but found unsupported file".into())
            }
        }
        
        Ok(FindConfig { target, root })
    }

    
    
    pub fn run_search(&self) {
    let results = search(&self);
    for path in results {
        if let Some(str) = path.to_str() {
            println!("{str}");
        }

    }
}
}

pub fn search(config: &FindConfig) -> Vec<PathBuf> {
        let mut results: Vec<PathBuf> = Vec::new();
        search_recursive(&config.target, &config.root, &mut results);
        results
    }

pub fn search_recursive(target: &str, root: &PathBuf, results: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let md = match fs::metadata(entry.path()) {
            Ok(md) => md,
            Err(_) => continue,
        };

        if md.is_dir() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_symlink() {
                    continue;
                }
            }

            search_recursive(target, &entry.path(), results);
        } 
        else {
            if entry.file_name().to_string_lossy().contains(target) {
                let ok_path = match entry.path().canonicalize() {
                    Ok(path) => path,
                    Err(_) => break
                };

                results.push(ok_path);
            }
        }
    }
}