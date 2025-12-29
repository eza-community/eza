use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, SystemTime};
use filetime::FileTime;
use tempfile::TempDir;

fn get_eza_binary() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("eza");
    path
}

// Helper to create files with specific modified times.
// Note: On most filesystems, created/birth time is set at creation and cannot
// be modified backwards. The filter uses max(modified, created), so files
// created during test execution will have recent created times even if their
// modified time is set to be old. Tests account for this behavior.
fn create_file_with_mtime(dir: &TempDir, filename: &str, age_secs: u64) -> PathBuf {
    let file_path = dir.path().join(filename);
    fs::File::create(&file_path).expect("Failed to create file");
    
    let now = SystemTime::now();
    let file_time = now
        .checked_sub(Duration::from_secs(age_secs))
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| FileTime::from_unix_time(d.as_secs() as i64, 0))
        .expect("Failed to calculate file time");
    
    filetime::set_file_times(&file_path, file_time, file_time).expect("Failed to set times");
    file_path
}

#[test]
fn test_since_filter_shows_recent_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    create_file_with_mtime(&temp_dir, "file1.txt", 30);
    create_file_with_mtime(&temp_dir, "file2.txt", 60);
    
    std::thread::sleep(Duration::from_millis(100));
    
    let eza = get_eza_binary();
    
    let output = Command::new(&eza)
        .arg("--since")
        .arg("5m")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute eza");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file1.txt"), "Should show file1.txt");
    assert!(stdout.contains("file2.txt"), "Should show file2.txt");
}

#[test]
fn test_since_filter_with_short_window() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    create_file_with_mtime(&temp_dir, "test.txt", 60);
    
    std::thread::sleep(Duration::from_millis(150));
    
    let eza = get_eza_binary();
    
    let output = Command::new(&eza)
        .arg("--since")
        .arg("50ms")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute eza");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("test.txt"), "Should not show with very short window after delay");
}

#[test]
fn test_since_filter_long_view() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    create_file_with_mtime(&temp_dir, "file.txt", 30);
    
    let eza = get_eza_binary();
    
    let output = Command::new(&eza)
        .arg("--since")
        .arg("1m")
        .arg("-l")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute eza");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file.txt"), "Should show in long view");
}

#[test]
fn test_since_filter_tree_view() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).expect("Failed to create subdir");
    
    create_file_with_mtime(&temp_dir, "root_file.txt", 30);
    
    let sub_file = subdir.join("sub_file.txt");
    fs::File::create(&sub_file).expect("Failed to create sub file");
    
    let eza = get_eza_binary();
    
    let output = Command::new(&eza)
        .arg("--since")
        .arg("1m")
        .arg("--tree")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute eza");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("root_file.txt"), "Should show root file in tree view");
    assert!(stdout.contains("subdir"), "Should show subdir");
    assert!(stdout.contains("sub_file.txt"), "Should show sub file in tree view");
}

#[test]
fn test_since_filter_invalid_duration() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_file_with_mtime(&temp_dir, "test.txt", 60);
    
    let eza = get_eza_binary();
    
    let output = Command::new(&eza)
        .arg("--since")
        .arg("invalid")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute eza");
    
    assert!(!output.status.success(), "Should fail with invalid duration");
}

#[test]
fn test_since_filter_one_line_view() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    create_file_with_mtime(&temp_dir, "file1.txt", 10);
    create_file_with_mtime(&temp_dir, "file2.txt", 20);
    
    let eza = get_eza_binary();
    
    let output = Command::new(&eza)
        .arg("--since")
        .arg("1m")
        .arg("-1")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute eza");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file1.txt"), "Should show file1.txt in one-line view");
    assert!(stdout.contains("file2.txt"), "Should show file2.txt in one-line view");
}

#[test]
fn test_since_filter_grid_view() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    
    create_file_with_mtime(&temp_dir, "a.txt", 15);
    create_file_with_mtime(&temp_dir, "b.txt", 25);
    create_file_with_mtime(&temp_dir, "c.txt", 35);
    
    let eza = get_eza_binary();
    
    let output = Command::new(&eza)
        .arg("--since")
        .arg("2m")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute eza");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("a.txt"), "Should show a.txt");
    assert!(stdout.contains("b.txt"), "Should show b.txt");
    assert!(stdout.contains("c.txt"), "Should show c.txt");
}
