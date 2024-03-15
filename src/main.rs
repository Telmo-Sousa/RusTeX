use std::process::Command;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        print_usage_and_exit();
    }
    let infile = args.remove(0);
    let outfile = args.remove(0);

    compile_latex(&infile, &outfile);
}

fn compile_latex(infile: &str, outfile: &str) {
    let status = Command::new("pdflatex")
        .arg("-interaction=nonstopmode")
        .arg("-output-directory")
        .arg(".")
        .arg(infile)
        .status()
        .expect("Failed to run pdflatex.");

    if !status.success() {
        eprintln!("Error: pdflatex process exited with non-zero status");
        std::process::exit(-1);
    }

    if let Err(e) = std::fs::rename("texput.pdf", outfile) {
        eprintln!("Error moving output PDF file: {}", e);
        std::process::exit(-1);
    }

    println!("PDF file generated successfully: {}", outfile);
}

fn print_usage_and_exit() {
    println!("USAGE: latex_compiler <INFILE> <OUTFILE>");
    std::process::exit(-1);
}

