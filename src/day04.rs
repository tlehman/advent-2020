/* 
--- Day 4: Passport Processing ---
You arrive at the airport only to realize that you grabbed your North Pole 
Credentials instead of your passport. While these documents are extremely 
similar, North Pole Credentials aren't issued by a country and therefore 
aren't actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though; a very long 
line has formed for the automatic passport scanners, and the delay could 
upset your travel itinerary.

Due to some questionable network security, you realize you might be able to 
solve both of these problems at the same time.

The automatic passport scanners are slow because they're having trouble 
detecting which passports have all required fields. The expected fields 
are as follows:

byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)
Passport data is validated in batch files (your puzzle input). Each 
passport is represented as a sequence of key:value pairs separated by 
spaces or newlines. Passports are separated by blank lines.

Here is an example batch file containing four passports:

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
The first passport is valid - all eight fields are present. The second 
passport is invalid - it is missing hgt (the Height field).

The third passport is interesting; the only missing field is cid, so it 
looks like data from North Pole Credentials, not a passport at all! Surely, 
nobody would mind if you made the system temporarily ignore missing cid 
fields. Treat this "passport" as valid.

The fourth passport is missing two fields, cid and byr. Missing cid is 
fine, but missing any other field is not, so this passport is invalid.

According to the above rules, your improved system would report 2 valid 
passports.

Count the number of valid passports - those that have all required fields. 
Treat cid as optional. In your batch file, how many passports are valid?

*/

struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>  // (Country ID)
}

impl Passport {
    pub fn from_vector(v: Vec<String>) -> Passport {            
        let mut byr: Option<String> = None;
        let mut iyr: Option<String> = None;
        let mut eyr: Option<String> = None;
        let mut hgt: Option<String> = None;
        let mut hcl: Option<String> = None;
        let mut ecl: Option<String> = None;
        let mut pid: Option<String> = None;
        let mut cid: Option<String> = None;

        for line in v {
            // key:value pairs are delimited by whitespace
            for kvp in line.split(" ") {
                let mut keyvalue = kvp.split(":").take(2);
                let key = keyvalue.next();
                if let Some(val) = keyvalue.next() {
                    let value = String::from(val);
                    if key == Some("byr") { byr = Some(value) }
                    else if key == Some("iyr") { iyr = Some(value) }
                    else if key == Some("eyr") { eyr = Some(value) }
                    else if key == Some("hgt") { hgt = Some(value) }
                    else if key == Some("hcl") { hcl = Some(value) }
                    else if key == Some("ecl") { ecl = Some(value) }
                    else if key == Some("pid") { pid = Some(value) }
                    else if key == Some("cid") { cid = Some(value) }
                }
            }
        }

        return Passport {
            byr, iyr, eyr, hgt,
            hcl, ecl, pid, cid
        };
    }

    pub fn valid(self: &Passport) -> bool {
        let invalid = self.byr.is_none() || self.iyr.is_none() ||
                      self.eyr.is_none() || self.hgt.is_none() ||
                      self.hcl.is_none() || self.ecl.is_none() ||
                      self.pid.is_none();
        return !invalid;
    }
}

#[test]
fn valid_passport_test() {
    let passport = Passport {
        ecl: Some(String::from("gry")),
        pid: Some(String::from("860033327")),
        eyr: Some(String::from("2020")),
        hcl: Some(String::from("#fffffd")),
        byr: Some(String::from("1937")),
        iyr: Some(String::from("2017")),
        cid: Some(String::from("147")),
        hgt: Some(String::from("183cm"))
    };
    assert_eq!(true, passport.valid());
}

#[test]
fn valid_northpole_credential_test() {
    let northpole_credential = Passport {
        hcl: Some(String::from("#ae17e1")),
        iyr: Some(String::from("2013")),
        eyr: Some(String::from("2024")),
        ecl: Some(String::from("brn")),
        pid: Some(String::from("760753108")),
        byr: Some(String::from("1931")),
        hgt: Some(String::from("179cm")),
        cid: None
    };
    assert_eq!(true, northpole_credential.valid());
}

#[test]
fn invalid_passport_test() {
    let passport = Passport {
        iyr: Some(String::from("2013")),
        ecl: Some(String::from("amb")),
        cid: Some(String::from("350")),
        eyr: Some(String::from("2023")),
        pid: Some(String::from("028048884")),
        hcl: Some(String::from("#cfa07d")),
        byr: Some(String::from("1929")),
        hgt: None
    };
    assert_eq!(false, passport.valid());
}

pub fn parse_batch_passports(batch_passports: &[&str]) -> Vec<Vec<String>> {
    let mut passports: Vec<Vec<String>> = Vec::new();
    // strategy: parse the input file line-by-line, then collect 
    // strings into a Vec<String>, and make a function to map a
    // Vec<String> -> Passport
    // use the push/from pattern used in day03.rs
    let mut current_passport: Vec<String> = Vec::new();
    for i in 0..batch_passports.len() {
        let line = batch_passports[i];
        if line == "" || i == batch_passports.len()-1 {
            passports.push(current_passport);
            current_passport = Vec::new();
        }
        current_passport.push(String::from(line));
    }
    return passports;
}

#[test]
fn batch_passports_parser_test() {
    // empty lines are passport delimiters
    let batch_passports: &[&str] = &[
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in"
    ];
    let passports = parse_batch_passports(batch_passports);
    assert_eq!(4, passports.len());
}

#[test]
fn passport_from_vector_test() {
    let batch_passports: &[&str] = &[
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in"
    ];
    let vector_passports = parse_batch_passports(batch_passports);
    let vector_passport1 = vector_passports.get(0).unwrap().clone();
    let vector_passport2 = vector_passports.get(1).unwrap().clone();
    let vector_passport3 = vector_passports.get(2).unwrap().clone();
    let vector_passport4 = vector_passports.get(3).unwrap().clone();
    // all this unwrap/clone here, is this the best way to do this?
    let passport1 = Passport::from_vector(vector_passport1);
    let passport2 = Passport::from_vector(vector_passport2);
    let passport3 = Passport::from_vector(vector_passport3);
    let passport4 = Passport::from_vector(vector_passport4);

    assert_eq!(true, passport1.valid());
    assert_eq!(false, passport2.valid());
    assert_eq!(true, passport3.valid());
    assert_eq!(false, passport4.valid());
}