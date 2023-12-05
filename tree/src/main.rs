use console::style;

const STAR: &str = r"
        |
       \|/
      --*--";

const TREE: &str = r"
       >o<
      >O<<<
     >>o>>*<
    >o<<<o<<<
   >>@>*<<O<<<
  >o>>@>>>o>o<<
 >*>>*<o<@<o<<<<
>o>o<<<O<*>>*>>O<
";

const BASE: &str = r"   _ __| |__ _";

const ORANGE: u8 = 166;

fn main() {
    let star: String = style(STAR).bold().yellow().to_string();
    let tree: String = TREE
        .chars()
        .map(|c| match c {
            '<' => style("<").bold().green().to_string(),
            '>' => style(">").bold().green().to_string(),
            'o' => style("o").bold().color256(ORANGE).to_string(),
            'O' => style("O").bold().blue().to_string(),
            '@' => style("@").bold().red().to_string(),
            '*' => style("*").bold().yellow().to_string(),
            ch => style(ch).bold().white().to_string(),
        })
        .collect();
    let base: String = style(BASE).bold().white().to_string();
    println!("{star}{tree}{base}");
}
