use std::process::Command;

pub struct TestFile {
    pub path: &'static str,
}

impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(self.path);
    }
}

pub struct OctaveCommand;

#[allow(dead_code)]
impl OctaveCommand {
    pub fn run(script: &str) -> String {
        let output = Command::new("octave")
            .arg("--eval")
            .arg(format!("run(\"{}\")", script))
            .output()
            .expect("Failed to execute Octave");
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

pub struct MatlabCommand;

#[allow(dead_code)]
impl MatlabCommand {
    pub fn run(script: &str) -> String {
        let output = Command::new("matlab")
            .arg("-batch")
            .arg(format!("run(\"{}\")", script))
            .output()
            .expect("Failed to execute MATLAB");
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}
