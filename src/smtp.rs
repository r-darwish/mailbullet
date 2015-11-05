use std::str::FromStr;

#[derive(Debug)]
pub enum SmtpCommand {
    Hello(String),
    MailFrom(String),
    RcptTo(String),
    Data
}

#[derive(Debug)]
pub enum ParsingError {
    UnknownCommand,
    BadParameters
}

impl FromStr for SmtpCommand {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<SmtpCommand, ParsingError> {
        let splitted : Vec<&str> = s.split(" ").collect();
        match splitted[0] {
            "HELO" => {
                match splitted.get(1) {
                    Some(domain) => {
                        return Ok(SmtpCommand::Hello(
                            String::from_str(domain).unwrap()));
                    }
                    _ => return Err(ParsingError::BadParameters)
                }
             },
             "MAIL" => {
                 match splitted.get(1) {
                     Some(subcommand) => {
                         if subcommand != &"FROM:" {
                             return Err(ParsingError::UnknownCommand);
                         }
                         match splitted.get(2) {
                             Some(address) => {
                                 return Ok(SmtpCommand::MailFrom(
                                     String::from_str(address).unwrap()));
                             }
                             _ => return Err(ParsingError::BadParameters)
                         }
                     }
                     _ => return Err(ParsingError::UnknownCommand)
                 }
              },
              "RCPT" => {
                  match splitted.get(1) {
                      Some(subcommand) => {
                          if subcommand != &"TO:" {
                              return Err(ParsingError::UnknownCommand);
                          }
                          match splitted.get(2) {
                              Some(address) => {
                                  return Ok(SmtpCommand::RcptTo(
                                      String::from_str(address).unwrap()));
                              }
                              _ => return Err(ParsingError::BadParameters)
                          }
                      }
                      _ => return Err(ParsingError::UnknownCommand)
                  }
               },
               "DATA" => {
                   return Ok(SmtpCommand::Data)
               }
            _ => { return Err(ParsingError::UnknownCommand) }
        }

    }
}
