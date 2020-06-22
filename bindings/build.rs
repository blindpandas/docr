winrt::build!(
  dependencies
    os
  types
    windows::globalization::*
    windows::media::ocr::*
    windows::security::cryptography::*
    windows::graphics::imaging::*
);

fn main() {
    build();
}
