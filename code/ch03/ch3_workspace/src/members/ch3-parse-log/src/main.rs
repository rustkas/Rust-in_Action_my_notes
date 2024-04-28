use ch3_parse_log::parse_log;

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";
    
      for line in log.lines(){
        let parse_result = parse_log(line);
        println!("{parse_result}");
      }
    }