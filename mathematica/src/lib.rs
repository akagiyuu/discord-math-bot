use run_script::ScriptOptions;

const WOLFRAMSCRIPT_WRAPPER: &str = include_str!("wolframscript_wrapper.sh");

pub enum OutputFormat {
    Plaintext,
    Image,
    Tex,
}
impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        match self {
            OutputFormat::Plaintext => "plaintex".to_string(),
            OutputFormat::Image => "image".to_string(),
            OutputFormat::Tex => "tex".to_string(),
        }
    }
}

pub fn eval(expression: String, format: OutputFormat) -> String {
    let options = ScriptOptions::new();
    let args = vec![
        "--code".to_string(),
        expression,
        "--format".to_string(),
        format.to_string(),
    ];
    let (_, output, _) = run_script::run(WOLFRAMSCRIPT_WRAPPER, &args, &options)
        .expect("Failed to evaluate expression");
    output.trim().to_string()
}
