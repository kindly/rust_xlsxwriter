// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! This example demonstrates adding a Textbox shape with text from a cell to a
//! worksheet.

use rust_xlsxwriter::{Shape, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Write some text to a cell.
    worksheet.write(1, 5, "This is some text")?;

    // Create a textbox shape and add some text from a cell.
    let textbox = Shape::textbox().set_text_link("=F2");

    // Insert a textbox in a cell.
    worksheet.insert_shape(1, 1, &textbox)?;

    // Save the file to disk.
    workbook.save("shape.xlsx")?;

    Ok(())
}
