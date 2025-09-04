use std::fmt::Display;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
  #[arg(short = 'c')]
  bytes: bool,
  #[arg(short = 'l')]
  lines: bool,
  #[arg(short = 'w')]
  words: bool,
  file_path: String,
}

#[derive(Debug, Default)]
struct FileInformation {
  file_name: Option<String>,
  bytes: Option<usize>,
  lines: Option<usize>,
  words: Option<usize>,
}

impl Display for FileInformation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{} {} {} {}",
      self.lines.map(|i| i.to_string()).unwrap_or_default(),
      self.words.map(|i| i.to_string()).unwrap_or_default(),
      self.bytes.map(|i| i.to_string()).unwrap_or_default(),
      self.file_name.clone().unwrap_or_default()
    )
  }
}

impl FileInformation {
  // FileInformation is monoidal so we create concat
  fn concat(self, fi: FileInformation) -> Self {
    FileInformation {
      file_name: self.file_name.or(fi.file_name),
      bytes: self.bytes.or(fi.bytes),
      lines: self.lines.or(fi.lines),
      words: self.words.or(fi.words),
    }
  }
}

struct FileContents(String);


fn mconcat(fis: Vec<FileInformation>) -> FileInformation {
  let mut accumulated_file_information = FileInformation {
    ..Default::default()
  };

  for i in fis {
    accumulated_file_information = accumulated_file_information.concat(i)
  }

  accumulated_file_information
}

fn get_words(FileContents(fc): &FileContents) -> FileInformation {
  let words: Vec<_> = fc.split_whitespace().collect();

  FileInformation {
    words: Some(words.iter().count()),
    ..Default::default()
  }
}

fn get_lines(FileContents(fc): &FileContents) -> FileInformation {
  let lines: Vec<_> = fc.split("\n").collect();

  FileInformation {
    lines: Some(lines.iter().count()),
    ..Default::default()
  }
}

fn get_bytes(FileContents(fc): &FileContents) -> FileInformation {
  FileInformation {
    bytes: Some(fc.as_bytes().len()),
    ..Default::default()
  }
}

fn main() {
  let args = Cli::parse();

  let file_contents =
    FileContents(std::fs::read_to_string(&args.file_path).expect("Could not read file"));

  let mut output: Vec<FileInformation> = vec![FileInformation {
    file_name: Some(args.file_path),
    ..Default::default()
  }];

  if args.bytes {
    output.push(get_bytes(&file_contents));
  }

  if args.lines {
    output.push(get_lines(&file_contents));
  }

  if args.words {
    output.push(get_words(&file_contents));
  }

  let file_information: FileInformation = mconcat(output);

  println!("{}", file_information);
}
