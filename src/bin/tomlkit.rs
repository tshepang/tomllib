extern crate tomllib;
use tomllib::parser::Parser;
use tomllib::types::{StrType, TOMLValue, TimeOffsetAmount, TimeOffset, DateTime, Str};

fn main() {
	let parser = Parser::new();
	let mut parser = parser.parse(r#"# This is a TOML document.

title = "TOML Example"

[owner]
name = "Tom Preston-Werner"
dob = 1979-05-27T07:32:00-08:00 # First class dates

[database]
server = "192.168.1.1"
ports = [ 8001, 8003, 8002 ]
connection_max = 5000
enabled = true

[servers]

  # Indentation (tabs and/or spaces) is allowed but not required
  [servers.alpha]
  ip = "10.0.0.1"
  dc = "eqdc10"

  [servers.beta]
  ip = "10.0.0.2"
  dc = "eqdc10"
[[flower]]
color = "blue"
type = "rose"
[[flower]]
color = "red"
type = "carnation"
  [flower.props]
    stuff = true
    junk = 4.35
  [[flower.power]]
  foo = "bar"
  
[clients]
data = [ ["gamma", "delta"], [1, 2] ]

# Line breaks are OK when inside arrays
hosts = [
  "alpha",
  "omega"
]"#);

  // let mut new_owner = String::new();
  // new_owner.push_str("Joel Self");
	println!("{}", parser.get_value("database.server".to_string()).unwrap());
  parser.set_value("database.server".to_string(),
    TOMLValue::String(Str::String("localhost".to_string()), StrType::Basic));
  println!("{}", parser.get_value("database.server".to_string()).unwrap());
  parser.set_value("database.connection_max".to_string(),
    TOMLValue::String(Str::String("100".to_string()), StrType::Literal));
  parser.set_value("title".to_string(), TOMLValue::Float(Str::String("4.567".to_string())));
  parser.set_value("owner.dob".to_string(), TOMLValue::DateTime(DateTime::new_string(
    "1981".to_string(), "04".to_string(), "15".to_string(),
    "08".to_string(), "08".to_string(), "00".to_string(), Some("005".to_string()),
    TimeOffset::Time(TimeOffsetAmount::new_string(
      "+".to_string(), "05".to_string(), "30".to_string()
    ))
  )));
  parser.print_keys_and_values();
  println!("{}", parser);
  // let (tmp, new_str) = parser.add_string(new_owner);
  // let parser = tmp;
  //parser.set_value("owner.name".to_string(),
    //TOMLValue::String(new_str, StrType::Basic));
 }