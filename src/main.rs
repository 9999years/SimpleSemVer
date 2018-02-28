use std::fmt;
use std::str;
use std::num;

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
// all-whitespace strings are Not considered blank, because i said so
// only implimented for string vecs i guess?
trait IsBlank {
    fn is_blank(&self) -> bool;
}

impl<'a> IsBlank for &'a str {
    fn is_blank(&self) -> bool {
        if self.len() == 0 {
            return true
        } else {
            return false
        }
    }
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

pub enum ParseSemVerError {
    MandatoryFieldNotNumber(num::ParseIntError),
}

//impl fmt::Display for ParseSemVerError {
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //match self {
            //ParseSemVerError::MandatoryFieldNotNumber
                //=> writeln!(f, "A mandatory field is not parseable as a uint32"),
            ////_ => writeln!(f, "Other error, not specified!"),
        //}
    //}
//}

impl From<num::ParseIntError> for ParseSemVerError {
    fn from(err: num::ParseIntError) -> ParseSemVerError {
        ParseSemVerError::MandatoryFieldNotNumber(err)
    }
}

impl<'p, 'm> str::FromStr for SemVer<'p, 'm> {
    type Err = ParseSemVerError;
    fn from_str(s: &str) -> Result<SemVer<'p, 'm>, ParseSemVerError> {
        //enum Field {
            //Major,
            //Minor,
            //Patch,
            //Prerelease,
            //Metadata,
        //}

        let mut ret: SemVer = Default::default();
        let mut current: String = "".to_string();
        let mut inx = 0;
        let mut fields: Vec<&str> = vec![];
        //turn our string into chunks
        for c in s.chars() {
            match c {
                '.' | '-' | '+'
                    => {
                        fields.push(&current[..]);
                        //each field starts with a ., -, or +
                        current = c.to_string();
                    },
                _   => current += &c.to_string(),
            }
        }

        for chunk in fields {
            println!("{}", chunk);
        }

        Ok(ret)

        // get next chunk function
        // parse (major, minor, patch) functions
        // generic parse pre/meta function, takes a char like + or -

        //{
            //let mut current_field = Field::Major;

            //let mut parse_chunk = |chunk: &String| -> Result<_, ParseSemVerError> {
                //match current_field {
                    //Field::Major | Field::Minor | Field::Patch
                        //=> {
                            //let num: u32 = try!((*chunk).parse());
                            //match current_field {
                                //Field::Major => {
                                    //ret.major = num;
                                    //current_field = Field::Minor;
                                //},
                                //Field::Minor => {
                                    //ret.minor = num;
                                    //current_field = Field::Patch;
                                //},
                                //Field::Patch => {
                                    //ret.patch = num;
                                    //current_field = Field::Prerelease;
                                //},
                                //_ => (),
                            //}
                    //},
                    //Field::Prerelease => {
                        //ret.prerelease.push(&chunk[..])
                    //},
                    //Field::Metadata   => {
                        //ret.metadata.push(&chunk[..])
                    //},
                //};
                //Ok(())
            //};

            //for c in s.chars() {
                //match c {
                    //'.' | '-' | '+'
                        //=> {
                            //try!(parse_chunk(&current));
                            //current = "".to_string();
                            //if c == '-' {
                                //let current_field = Field::Prerelease;
                            //} else if c == '+' {
                                //let current_field = Field::Metadata;
                            //}

                        //},
                    //_   => current += &c.to_string(),
                //}
            //}
            //try!(parse_chunk(&current));
        //}
        //Ok(ret)
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
    println!("{}", "1.0.0-5".parse::<SemVer>()
        .unwrap_or(Default::default()).to_string());
}
