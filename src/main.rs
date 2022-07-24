use clap::Parser;
use std::{collections::HashMap, error::Error};
use tx_processing_engine::{
    client::client::Client, tx::execution_engine::Engine, util::args::Args,
    util::csv_output_writer, util::file_parser::parse,
};

/// main method processing file
fn main() -> Result<(), Box<dyn Error>> {
    let file = Args::parse().file;
    let should_log_errors = Args::parse().log_errors;
    let account_state = process_file(file, should_log_errors)?;
    csv_output_writer::write_csv_to_console(&account_state)?;
    Ok(())
}

fn process_file(file: String, log_errors: bool) -> Result<HashMap<u16, Client>, Box<dyn Error>> {
    let mut exec_engine = Engine::initialize();
    // TODO handle deserialization error for a malformed record
    for result in parse(file)?.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        match result {
            Ok(tx) => match exec_engine.execute_transaction(tx) {
                Ok(_) => (),
                Err(err) => {
                    if log_errors {
                        eprintln!("{}", err);
                    }
                }
            },
            Err(err) => {
                if log_errors {
                    eprintln!("skipped invalid TX: {}", err);
                }
            }
        }
    }
    Ok(exec_engine.get_account_state_owned())
}

#[cfg(test)]
mod tests {
    use crate::process_file;

    #[test]
    fn case_01() {
        let result = process_file(String::from("./test_input/case_01.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_02() {
        let result = process_file(String::from("./test_input/case_02.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_03() {
        let result = process_file(String::from("./test_input/case_03.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_04() {
        let result = process_file(String::from("./test_input/case_04.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_05() {
        let result = process_file(String::from("./test_input/case_05.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_06() {
        let result = process_file(String::from("./test_input/case_06.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.5, acc.get_available());
        assert_eq!(0.5, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_07() {
        let result = process_file(String::from("./test_input/case_07.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(1.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_08() {
        let result = process_file(String::from("./test_input/case_08.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_09() {
        let result = process_file(String::from("./test_input/case_09.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_10() {
        let result = process_file(String::from("./test_input/case_10.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.9, acc.get_available());
        assert_eq!(0.9, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_11() {
        let result = process_file(String::from("./test_input/case_11.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(1.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_12() {
        let result = process_file(String::from("./test_input/case_12.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // account should be locked
    #[test]
    fn case_13() {
        let result = process_file(String::from("./test_input/case_13.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(2.0, acc.get_available());
        assert_eq!(2.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    #[test]
    fn case_14() {
        let result = process_file(String::from("./test_input/case_14.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(1.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_15() {
        let result = process_file(String::from("./test_input/case_15.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_16() {
        let result = process_file(String::from("./test_input/case_16.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_17() {
        let result = process_file(String::from("./test_input/case_17.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    #[test]
    fn case_18() {
        let result = process_file(String::from("./test_input/case_18.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    #[test]
    fn case_19() {
        let result = process_file(String::from("./test_input/case_19.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    #[test]
    fn case_20() {
        let result = process_file(String::from("./test_input/case_20.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    #[test]
    fn case_21() {
        let result = process_file(String::from("./test_input/case_21.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }
    #[test]
    fn case_22() {
        let result = process_file(String::from("./test_input/case_22.csv"), true).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.5, acc.get_available());
        assert_eq!(0.5, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }
}
