// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::Workbook;

mod common;

// Test case to demonstrate XXX
fn create_new_xlsx_file(filename: &str) {
    let mut workbook = Workbook::new(filename);
    workbook.close();
}

#[test]
fn compare_against_excel() {
    let testcase = "simple00";

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames(testcase);
    create_new_xlsx_file(&xlsxwriter_file);
    common::assert_eq(&excel_file, &xlsxwriter_file);
}
