use std::fmt;
use std::str;

#[derive(Eq, PartialEq, Clone, Hash, Default, Debug)]
pub struct SemVer<'p, 'm> {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: Vec<&'p str>,
    metadata:   Vec<&'m str>,
}

// figures out if a type is "empty", having no contents
// for example, a Vec["", "", ""] has elements, but they are all blank,
// so it is blank
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

impl<'p, 'm> str::FromStr for SemVer<'p, 'm> {
    type Err = ParseSemVerError;
    fn from_str(s: &str) -> Result<SemVer<'p, 'm>, ParseSemVerError> {
        enum Field {
            major,
            minor,
            patch,
            prerelease,
            metadata,
        }

        let mut current_field = Field::major;
        let mut ret: SemVer = Default::default();
        let mut current: String = "".to_string();

        let mut parse_chunk = |chunk: &String| -> Result<_, ParseSemVerError> {
            match current_field {
                Field::major | Field::minor | Field::patch
                    => {
                        let num: u32 = match (*chunk).parse() {
                            Ok(val) => val,
                            Err(err) => return Err(
                                ParseSemVerError { error: -1 }
                            ),
                        };
                        match current_field {
                            Field::major => {
                                ret.major = num;
                                current_field = Field::minor;
                            },
                            Field::minor => {
                                ret.minor = num;
                                current_field = Field::patch;
                            },
                            Field::patch => {
                                ret.patch = num;
                                current_field = Field::prerelease;
                            },
                            _ => (),
                        }
                    }
                Field::prerelease => { ret.prerelease.push(&(current.clone())[..]) },
                //Field::metadata   => { ret.metadata.push(&current[..]) },
                _ => (),
            };
            Ok(())
        };

        for c in s.chars() {
            match c {
                '.' | '-' | '+'
                    => {
                        match parse_chunk(&current) {
                            Ok(val) => val,
                            Err(err) => return Err(err),
                        }
                        current = "".to_string();
                        if c == '-' {
                            current_field = Field::prerelease;
                        } else if c == '+' {
                            current_field = Field::metadata;
                        }

                    },
                _   => current += &c.to_string(),
            }
        }
        parse_chunk(&current);
        print!("\n");
        Ok(ret)
    }
}

pub struct ParseSemVerError {
    error: i32
}

impl fmt::Display for ParseSemVerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        enum Errors {
            field_not_number
        }
        unimplemented!();
    }
}

impl<'p, 'm> fmt::Display for SemVer<'p, 'm> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = format!("{}.{}.{}",
            self.major, self.minor, self.patch);

        if !self.prerelease.is_blank() {
            output += &format!("-{}", self.prerelease.join("."))[..];
        }

        if !self.metadata.is_blank() {
            output += &format!("+{}", self.metadata.join("."))[..];
        }

        write!(f, "{}", output)
    }
}

impl<'p, 'm> SemVer<'p, 'm> {
}

fn main() {
    let ver = SemVer {
        minor: 100,
        prerelease: vec!["hahhghagh", "2016"],
        .. Default::default()
    };
    ver.to_string().as_str().parse::<SemVer>();
    println!("{}", ver.to_string());
}
