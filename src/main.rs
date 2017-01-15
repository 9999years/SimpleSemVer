pub struct SemVer<'p, 'm> {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: Vec<&'p str>,
    metadata:   Vec<&'m str>,
}

trait IsBlank {
    fn is_blank(&self) -> bool;
}

impl<'a> IsBlank for Vec<&'a str> {
    fn is_blank(&self) -> bool {
        if self.len() == 0 {
            //empty
            return true;
        } else {
            //iterate
            for el in self {
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

    pub fn new() -> SemVer<'p, 'm> {
        SemVer {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: vec![],
            metadata: vec![],
        }
    }

    pub fn into_string(&self) -> String {
        let mut output: String = format!("{}.{}.{}",
            self.major, self.minor, self.patch);

        if !self.prerelease.is_blank() {
            output += &("-".to_owned() + &self.prerelease.join(".")[..]);
        }

        if !self.metadata.is_blank() {
            output += &("+".to_owned() + &self.metadata.join(".")[..]);
        }

        output
    }

    fn from_string(&self, input: &str) -> SemVer<'p, 'm> {
        for part in input.split(&['.', '+', '-',][..]) {
            println!("{}", part);
        }
        SemVer::new()
    }
}

fn main() {
    let ver = SemVer {
        minor: 100,
        prerelease: vec!["hahhghagh", "2016"],
        metadata: vec!["comp128218218"],
        .. SemVer::new()
    };
    ver.from_string(ver.into_string().as_str());
    println!("{}", ver.into_string());
}
