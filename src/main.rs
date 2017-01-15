struct SemVer<'p, 'm> {
    major: i32,
    minor: i32,
    patch: i32,
    prerelease: Vec<&'p str>,
    metadata:   Vec<&'m str>,
}

impl<'s> Vec<&'s str> {
    fn is_blank(&self, arr: Vec<&str>) -> bool {
        if arr.len() == 0 {
            //empty
            return true;
        } else {
            //iterate
            for el in arr {
                if el.len() != 0 {
                    //got a string
                    return false;
                }
            }
            return true;
        }
    }
}

impl<'p, 'm> SemVer<'p, 'm> {

    fn into_string(&self) -> String {
        let mut output: String = format!("{}.{}.{}",
            self.major, self.minor, self.patch);

        if !self.prerelease {
            output += &("-".to_owned() + &self.prerelease.join(".")[..]);
        }

        if !self.metadata.is_empty() {
            output += &("+".to_owned() + &self.metadata.join(".")[..]);
        }

        output
    }

    //fn from_string(&self, input: str) -> SemVer<'p, 'm> {
    //}

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
