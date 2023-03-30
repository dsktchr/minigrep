use std::env;
use std::error::Error;
use std::{fs::File, io::Read};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ファイルをオープンする
    let mut f = File::open(config.filename)?;

    let mut content = String::new();

    f.read_to_string(&mut content)?;

    let results = if config.case_insensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("合致するテキストは: \n{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    content
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        // 探したい文字列
        let query = match args.next() {
            Some(v) => v,
            None => return Err("値を取得できませんでした"),
        };
        // 探したいファイル
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("値を取得できませんでした"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

//tests
#[cfg(test)]
mod tests {
    use crate::{run, search, search_case_insensitive, Config};

    #[test]
    fn build_error_config() {
        let lacked_args: Vec<String> = vec!["hoge".to_string(), "fuga".to_string()];
        let config = Config::new(&lacked_args);

        assert_eq!("引数の数が足りません", config.unwrap_err());
    }

    #[test]
    fn build_success_config() {
        let query = "fuga".to_string();
        let filename = "piyo.text".to_string();

        let enough_args: Vec<String> = vec!["hoge".to_string(), query.clone(), filename.clone()];
        let r = Config::new(&enough_args);

        let a = r.unwrap();

        // 同一のクエリか
        assert_eq!(&query, &a.query);

        // 同一のファイル名か
        assert_eq!(&filename, &a.filename);
    }

    #[test]
    fn is_exists_file() {
        let config = Config {
            query: "hoge".to_string(),
            filename: "poem.txt".to_string(),
        };
        let result = run(config);

        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn is_not_exists_file() {
        let config = Config {
            query: "hoge".to_string(),
            filename: "poetry.txt".to_string(),
        };
        let result = run(config);

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
