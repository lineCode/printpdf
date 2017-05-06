extern crate error_chain;

use super::*;

error_chain! {

    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        /*FileError(::std::io::Error);*/
    }

    links {
        PDFError(pdf_error::Error, pdf_error::ErrorKind);
        IndexError(index_error::Error, index_error::ErrorKind);
    }

    errors {
       /*PdfFileError {
           description("Selected local file is not a PDF file!")
           display("Could not load file")
       }*/
    }
}