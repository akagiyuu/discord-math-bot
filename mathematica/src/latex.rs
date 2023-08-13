mod processor;

use super::image_format::ImageFormat;
use crate::TEMP_FOLDER;
use anyhow::Result;
use run_script::ScriptOptions;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const WOLFRAMSCRIPT_WRAPPER: &str = r#"
# env/bin/bash

cd /tmp

TEMP=`getopt -o c:f: --long code:,format: -- "$@"`
eval set -- "$TEMP"

while true ; do
    case "$1" in
        -c|--code)
            code=$2;
            shift 2;;
        -f|--format)
            case "$2" in
                "plaintex"|"tex") format=$2 ; shift 2 ;;
                               *) format="plaintex" ; shift 2 ;;
            esac ;;
        --) shift ; break ;;
        *) echo "Internal error!" ; exit 1 ;;
    esac
done

if [[ "$format" == "plaintex" ]]
then
    echo $(wolframscript -c "$code")
else
    base_file_name=$(date +%s)

    content=$(wolframscript -c "$code" -format TeX)
    match="documentclass{article}"
    replace="documentclass[border=2pt,varwidth]{standalone}"
    content=${content/"$match"/"$replace"}
    content="%${content#*%}"

    echo "$content" > "$base_file_name.tex"
    pdflatex "$base_file_name.tex" > /dev/null
    convert -density 300 $base_file_name.pdf -quality 90 -background white -alpha remove -alpha off "$base_file_name.png" > /dev/null
    echo "$base_file_name.png"
fi
"#;

fn get_random_unique_name() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
}

pub struct TexFile {
    path: PathBuf,
}
impl TexFile {
    pub fn create_with_random_name(content: &[u8]) -> Result<Self> {
        let path = PathBuf::from(format!("{}/{}.tex", TEMP_FOLDER, get_random_unique_name()));

        let content = processor::trim(content)?;

        println!("{:?}", path.to_string_lossy().to_string());
        let mut latex_file = File::create(path.to_string_lossy().to_string())?;
        latex_file.write_all(content.as_bytes())?;
        println!("{:?}", latex_file);

        Ok(Self { path })
    }
    fn compile(&self) -> Result<PathBuf> {
        let options = ScriptOptions::new();
        let args = vec![
            "-c".to_string(),
            "Solve[x^2 + 2 x - 7 == 0, x]".to_string(),
            "-f".to_string(),
            "plaintex".to_string(),
        ];
        let (_, output, _) = run_script::run(WOLFRAMSCRIPT_WRAPPER, &args, &options).unwrap();
        println!("{}", output);
        Command::new("pdflatex")
            .current_dir(TEMP_FOLDER)
            .arg(self.path.to_string_lossy().to_string())
            .output()?;
        Ok(self.path.with_extension("pdf"))
    }
    pub fn create_img(&mut self, image_format: ImageFormat) -> Result<PathBuf> {
        let img_path = self.path.with_extension(image_format.as_str());
        let pdf_path = self.compile()?;
        Command::new("convert")
            .current_dir(TEMP_FOLDER)
            .args(["-density", "300"])
            .arg(pdf_path.to_string_lossy().to_string())
            .args([
                "-quality",
                "90",
                "-background",
                "white",
                "-alpha",
                "remove",
                "-alpha",
                "off",
            ])
            .arg(img_path.to_string_lossy().to_string())
            .output()?;
        Ok(img_path)
    }
}
