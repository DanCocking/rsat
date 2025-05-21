use std::{
    collections::BTreeSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct CNF {
    pub nvar: i32,
    pub nclause: i32,
    pub clauses: BTreeSet<BTreeSet<i32>>,
}

impl CNF {
    pub fn from_dirac_file(path: String) -> Result<CNF, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        let bytes_read = reader.read_line(&mut first_line)?;

        if bytes_read == 0 {
            println!("File is empty!");
            return Err("File is empty".into());
        }
        let mut cnf = match first_line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["p", "cnf", nvar, nclause] => {
                let nvar = nvar.parse::<i32>()?;
                let nclause = nclause.parse::<i32>()?;
                CNF {
                    nvar,
                    nclause,
                    clauses: BTreeSet::new(),
                }
            }
            _ => panic!("Bad input file first line"),
        };
        for line in reader.lines() {
            let line = line?;
            let literals = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .take_while(|&x| x != 0)
                .collect::<BTreeSet<i32>>();
            if !literals.is_empty() {
                cnf.clauses.insert(literals);
            }
        }
        Ok(cnf)
    }
}
