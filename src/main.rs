fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        std::process::Command::new("pdftotext")
            .arg(&args[1])
            .status()
            .expect("Error while reading the file!");
        if args.len() == 3 {
            std::process::Command::new("espeak")
                .args(["-f", &args[1].replace("pdf", "txt"), "-w", &args[2]])
                .status()
                .expect("Error while reading the file!");
        } else {
            std::process::Command::new("espeak")
                .args([
                    "-v",
                    "en-gb-scotland",
                    "-f",
                    &args[1].replace("pdf", "txt"),
                    "-w",
                    &args[1].replace("pdf", "wav"),
                ])
                .status()
                .expect("Error while reading the file!");
        }
        std::process::Command::new("rm")
            .args([args[1].replace(".pdf", ".txt")])
            .status()
            .expect("Error while reading the file!");
        println!("Done!")
    } else {
        println!("Usage: pdftoaudio <filename>  <output file.wav>-optional")
    }
}
