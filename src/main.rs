struct SemVer<'p, 'm> {
    major: i32,
    minor: i32,
    patch: i32,
    prerelease: Vec<&'p str>,
    metadata:   Vec<&'m str>,
}

impl<'p, 'm> SemVer<'p, 'm> {
    fn into_string(&self) -> String {
        let output: String = format!("{}.{}.{}",
            self.major, self.minor, self.patch);
        if !self.prerelease.is_empty() {
            output += "-".to_string() + self.prerelease.join(".");
        }
        if !self.metadata.is_empty() {
            output += "+" + self.metadata.join(".");
        }
    }

    fn from_string(&self, input: str) -> SemVer {
    }

    fn new() -> SemVer<'p, 'm> {
        SemVer {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: vec![""],
            metadata: vec![""],
        }
    }
}

fn main() {
    println!("Hello!");
    let ver: SemVer = SemVer::new();
    println!("{}", ver.into_string());
}
