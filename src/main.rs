use pandoc::OutputKind;
use std::path::Path;

fn main() {
    let path_to_source = Path::new("artifacts/template.md");

    let mut pandoc = pandoc::new();
    pandoc.add_input(path_to_source);
    println!("Path added: {}", path_to_source.display());
    pandoc.set_output(OutputKind::File("artifacts/resume.pdf".into()));

    pandoc.execute().unwrap();
    println!("Done");
}
