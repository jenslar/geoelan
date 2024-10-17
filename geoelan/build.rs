use std::process::Command;
fn main() {
    // Compile full manual asciidoc -> pdf, html+txt
    // geoelan only bundles the pdf + txt
    println!("cargo:rerun-if-changed=../doc/");

    // 1. MD -> TXT, pandoc concatenate from ../doc/markdown:
    //    pandoc --defaults ../doc/pandoc/include.yaml -f gfm -t plain -o ../doc/txt/geoelan.txt
    Command::new("pandoc")
        .args(&[
            "--defaults",
            "../doc/pandoc/include.yaml",
            "-f",
            "gfm",
            "-t",
            "plain",
            "-o",
            "../doc/txt/geoelan.txt",
        ])
        .output()
        .expect("(!) Failed to execute process (MD -> TXT). Is pandoc in PATH?");

    // 2. MD -> ADOC, FULL pandoc concatenate from ../doc/markdown:
    //    pandoc --defaults ../doc/pandoc/include.yaml -f gfm -t asciidoc -o ../doc/asciidoc/geoelan.adoc
    Command::new("pandoc")
        .args(&[
            "--defaults",
            "../doc/pandoc/include.yaml",
            "-f",
            "gfm",
            "-V",
            "documentclass=report",
            "--toc",
            "-t",
            "asciidoc",
            "-o",
            "../doc/asciidoc/geoelan.adoc",
        ])
        .output()
        .expect("(!) Failed to execute process (MD -> ADOC). Is pandoc in PATH?");

    // 3. ADOC -> PDF, asciidoctor-pdf for full doc geoelan.adoc
    //    asciidoctor-pdf -a pdf-theme=../doc/asciidoc/theme/geoelan-theme.yml -a pdf-fontsdir=../doc/asciidoc/fonts/ -o ../doc/pdf/geoelan.pdf ../doc/asciidoc/geoelan.adoc
    Command::new("asciidoctor-pdf") // must be in path, full adoc -> pdf
        .args(&[
            "-a",
            "pdf-theme=../doc/asciidoc/theme/geoelan-theme.yml",
            "-a",
            "pdf-fontsdir=../doc/asciidoc/fonts/",
            "-o",
            "../doc/pdf/geoelan.pdf",
            "../doc/asciidoc/geoelan.adoc",
        ])
        .output()
        .expect("Failed to execute process (ADOC -> PDF, full). Is asciidoctor-pdf in PATH?");

    // 5. MD -> HTML, mdbook
    //    mdbook build -d ../mdbook/ ../doc/markdown/
    Command::new("mdbook") // must be in path, a4 adoc -> pdf
        .args(&[
            "build",
            "-d",
            "../mdbook/", // relative to src dir ../doc/markdown...
            "../doc/markdown/",
        ])
        .output()
        .expect("Failed to execute process. Is mdbook in PATH?");
}
