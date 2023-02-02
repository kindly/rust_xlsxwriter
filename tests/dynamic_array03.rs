// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test future function with explicit xlfn.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_formula(0, 0, "=1+_xlfn.XOR(1)")?;
    worksheet.set_formula_result(0, 0, "2");

    workbook.save(filename)?;

    Ok(())
}

// Test future function with implicit xlfn.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.use_future_functions(true);
    worksheet.write_formula(0, 0, "=1+XOR(1)")?;
    worksheet.set_formula_result(0, 0, "2");

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_dynamic_array03_1() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array03")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array03_2() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array03")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
