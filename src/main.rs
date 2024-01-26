use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Caminho para o arquivo CSV
    let path = Path::new("Files/test csv.csv");

    // Abra o arquivo para leitura com manipulação de erros
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Iterar sobre as linhas do arquivo
    for line in reader.lines() {
        // Manipular o conteúdo da linha (aqui, apenas imprimindo)
        if let Ok(line_content) = line {
            println!("{}", line_content);
            
            // Aqui, você pode dividir a linha em colunas usando a vírgula como delimitador
            let columns: Vec<&str> = line_content.split(',').collect();
            
            // Agora você pode trabalhar com as colunas como achar necessário
            // Exemplo: println!("Primeira coluna: {}", columns[0]);
        }
    }

    Ok(())
}
