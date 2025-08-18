use crate::lily;

/// Prints a startup message to the CLI
pub fn cli_startup_message(lily: &lily::Cms) {
    let msg: Vec<String> = vec![
        logo().to_owned(),
        lily::core::env::get_lily_product_id(),
        lily.get_address(),
    ];
    println!("{}", msg.join("\n"));
}

/// Returns a raw string of "LILY CMS" in ascii art
/// https://patorjk.com/software/taag/#p=display&f=Univers&t=lily%20cms
fn logo() -> &'static str {
    r#"
88  88  88
88      88
88      88
88  88  88  8b       d8      ,adPPYba,  88,dPYba,,adPYba,   ,adPPYba,
88  88  88  `8b     d8'     a8"     ""  88P'   "88"    "8a  I8[    ""
88  88  88   `8b   d8'      8b          88      88      88   `"Y8ba,
88  88  88    `8b,d8'       "8a,   ,aa  88      88      88  aa    ]8I
88  88  88      Y88'         `"Ybbd8"'  88      88      88  `"YbbdP"'
                d8'
               d8'
"#
}
