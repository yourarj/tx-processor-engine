use clap::Parser;
use std::{collections::HashMap, error::Error};
use tx_processing_engine::{
    client::client::Client, tx::execution_engine::Engine, util::args::Args,
    util::csv_output_writer, util::file_parser::parse,
};

/// main method processing file
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let account_state = process_file(args.file, args.log_errors, args.unsafe_mode)?;
    csv_output_writer::write_csv_to_console(&account_state)?;
    Ok(())
}

/// file processing
/// input: String: filepath
/// input: bool: should print error
fn process_file(
    file: String,
    log_errors: bool,
    unsafe_mode: bool,
) -> Result<HashMap<u16, Client>, Box<dyn Error>> {
    let mut exec_engine = Engine::initialize();

    for result in parse(file, unsafe_mode)?.deserialize() {
        match result {
            Ok(tx) => {
                if let Err(err) = exec_engine.execute_transaction(tx) {
                    if log_errors {
                        eprintln!("PROCESSING_TX_ERROR: {}", err);
                    }
                }
            }
            Err(err) => {
                if log_errors {
                    eprintln!("INVALID_CSV_ROW: {}", err);
                }
            }
        }
    }
    Ok(exec_engine.get_account_state_owned())
}

#[cfg(test)]
mod tests {
    use crate::process_file;

    // 1. case_01
    // - genesis -> deposit
    // - should succeed
    #[test]
    fn case_01() {
        let result = process_file(String::from("./test_input/case_01.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 2. case_02
    // - genesis -> withdraw
    // - ignore
    #[test]
    fn case_02() {
        let result = process_file(String::from("./test_input/case_02.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 3. case_03
    //    - genesis -> dispute
    //    - ignore
    #[test]
    fn case_03() {
        let result = process_file(String::from("./test_input/case_03.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 4. case_04
    //    - genesis -> chargeback
    //    - ignore
    #[test]
    fn case_04() {
        let result = process_file(String::from("./test_input/case_04.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 5. case_05
    // - genesis -> resolve
    // - ignore
    #[test]
    fn case_05() {
        let result = process_file(String::from("./test_input/case_05.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 6. case_06
    // - genesis -> deposit -> withdraw
    // - succeed
    #[test]
    fn case_06() {
        let result = process_file(String::from("./test_input/case_06.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.5, acc.get_available());
        assert_eq!(0.5, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 7. case_07
    // - genesis -> deposit -> dispute
    // - succeed
    #[test]
    fn case_07() {
        let result = process_file(String::from("./test_input/case_07.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(1.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 8. case_08
    //    - genesis -> deposit -> resolve
    //    - ignore
    #[test]
    fn case_08() {
        let result = process_file(String::from("./test_input/case_08.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 9. case_09
    //    - genesis -> deposit -> chargeback
    //    - ignore
    #[test]
    fn case_09() {
        let result = process_file(String::from("./test_input/case_09.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 10. case_10
    //    - genesis -> deposit -> withdraw -> dispute
    //    - ignore
    #[test]
    fn case_10() {
        let result = process_file(String::from("./test_input/case_10.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.9, acc.get_available());
        assert_eq!(0.9, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 11. case_11
    //    - genesis -> deposit -> dispute -> dispute
    //    - ignore
    #[test]
    fn case_11() {
        let result = process_file(String::from("./test_input/case_11.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(1.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 12. case_12
    //    - genesis -> deposit -> dispute -> resolve
    //    - succeed
    #[test]
    fn case_12() {
        let result = process_file(String::from("./test_input/case_12.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // account should be locked
    // 13. case_13
    //    - genesis -> deposit -> dispute -> chargeback
    //    - succeed
    #[test]
    fn case_13() {
        let result = process_file(String::from("./test_input/case_13.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(2.0, acc.get_available());
        assert_eq!(2.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    // 14. case_14
    //    - genesis -> deposit -> dispute -> withdraw
    //    - ignore
    #[test]
    fn case_14() {
        let result = process_file(String::from("./test_input/case_14.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(1.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 15. case_15
    //    - genesis -> deposit -> dispute -> resolve -> dispute
    //    - ignore
    #[test]
    fn case_15() {
        let result = process_file(String::from("./test_input/case_15.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 16. case_16
    //    - genesis -> deposit -> dispute -> resolve -> resolve
    //    - ignore
    #[test]
    fn case_16() {
        let result = process_file(String::from("./test_input/case_16.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 17. case_17
    //    - genesis -> deposit -> dispute -> resolve -> chargeback
    //    - ignore
    #[test]
    fn case_17() {
        let result = process_file(String::from("./test_input/case_17.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(1.0, acc.get_available());
        assert_eq!(1.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // 18. case_18
    //    - genesis -> deposit -> dispute -> chargeback -> chargeback
    //    - ignore
    #[test]
    fn case_18() {
        let result = process_file(String::from("./test_input/case_18.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    // 19. case_19
    //    - genesis -> deposit -> dispute -> chargeback -> deposit
    //    - ignore
    #[test]
    fn case_19() {
        let result = process_file(String::from("./test_input/case_19.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    // 20. case_20
    //    - genesis -> deposit -> dispute -> chargeback -> withdraw
    //    - ignore
    #[test]
    fn case_20() {
        let result = process_file(String::from("./test_input/case_20.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    // 21. case_21
    //    - genesis -> deposit -> dispute -> chargeback -> dispute
    //    - ignore
    #[test]
    fn case_21() {
        let result = process_file(String::from("./test_input/case_21.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(true, acc.is_locked());
    }

    // 22. case_22
    //    - genesis -> deposit -> partial_withdraw -> full_withdraw
    //    - ignore
    #[test]
    fn case_22() {
        let result = process_file(String::from("./test_input/case_22.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.5, acc.get_available());
        assert_eq!(0.5, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // try depositing negative amount
    // 23. case_23
    //    - deposit negative amount
    //    - ignore
    #[test]
    fn case_23() {
        let result = process_file(String::from("./test_input/case_23.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // try withdrawing negative amount
    #[test]
    fn case_24() {
        let result = process_file(String::from("./test_input/case_24.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // add deposit two amount exactly f64::MAX
    #[test]
    fn case_25() {
        let result = process_file(String::from("./test_input/case_25.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(f64::MAX, acc.get_available());
        assert_eq!(f64::MAX, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // tx amount larger that max size of f64
    #[test]
    fn case_26() {
        let result = process_file(String::from("./test_input/case_26.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // try deposit without amount
    #[test]
    fn case_27() {
        let result = process_file(String::from("./test_input/case_27.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // try withdrawal without amount
    #[test]
    fn case_28() {
        let result = process_file(String::from("./test_input/case_28.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }

    // deposit without client id
    #[test]
    fn case_29() {
        let result = process_file(String::from("./test_input/case_29.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // deposit without transaction id
    #[test]
    fn case_30() {
        let result = process_file(String::from("./test_input/case_30.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // withdrawal without client id
    #[test]
    fn case_31() {
        let result = process_file(String::from("./test_input/case_31.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // withdrawal without transaction id
    #[test]
    fn case_32() {
        let result = process_file(String::from("./test_input/case_32.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // dispute with client id
    #[test]
    fn case_33() {
        let result = process_file(String::from("./test_input/case_33.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // dispute without tx id
    #[test]
    fn case_34() {
        let result = process_file(String::from("./test_input/case_34.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // resolve without client id
    #[test]
    fn case_35() {
        let result = process_file(String::from("./test_input/case_35.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // resolve without tx id
    #[test]
    fn case_36() {
        let result = process_file(String::from("./test_input/case_36.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // chargeback without client id
    #[test]
    fn case_37() {
        let result = process_file(String::from("./test_input/case_37.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // chargeback without tx id
    #[test]
    fn case_38() {
        let result = process_file(String::from("./test_input/case_38.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }

    // withdraw amount exactly f64::MAX
    #[test]
    fn case_39() {
        let result = process_file(String::from("./test_input/case_39.csv"), true, false).unwrap();
        let acc = result.get(&1).unwrap();
        assert_eq!(1, acc.get_client());
        assert_eq!(0.0, acc.get_available());
        assert_eq!(0.0, acc.get_total());
        assert_eq!(0.0, acc.get_held());
        assert_eq!(false, acc.is_locked());
    }
    // client id smaller than 0 (negative)
    #[test]
    fn case_40() {
        let result = process_file(String::from("./test_input/case_40.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }
    // clint id larger than f16::MAX
    #[test]
    fn case_41() {
        let result = process_file(String::from("./test_input/case_41.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }
    // tx id smaller than 0 (negative)
    #[test]
    fn case_42() {
        let result = process_file(String::from("./test_input/case_42.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }
    // tx id larger than u32::MAX
    #[test]
    fn case_43() {
        let result = process_file(String::from("./test_input/case_43.csv"), true, false).unwrap();
        // tx should be discarded
        assert_eq!(true, result.get(&1).is_none());
    }
}
