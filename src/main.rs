use addressbook::app;

const FILENAME: &str = "src/db/addressbook.txt";

fn main() {
    app::run(FILENAME);
}