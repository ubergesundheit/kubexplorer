use clap::{App, Arg};
use std::str::FromStr;

pub fn parse_user_input() -> UserArgs {
    let matches = App::new("KubEx - Kubernetes Explorer")
        .version("0.1.0")
        .author("Pavel Pscheidl <pavelpscheidl@gmail.com>")
        .about("Discovers unused ConfigMaps and Secrets")
        .arg(
            Arg::with_name("KUBECONFIG")
                .short("k")
                .long("kubeconfig")
                .value_name("PATH_TO_KUBECONFIG")
                .help("Path to a KUBECONFIG file. When not set, env is used.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("NAMESPACE")
                .short("n")
                .long("namespace")
                .value_name("NAMESPACE")
                .help("Namespace to search in.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Output format. Plain by default.")
                .possible_values(&["yaml", "json", "plain"])
                .default_value("plain")
                .takes_value(true),
        )
        // .arg(
        //     Arg::with_name("KIND")
        //         .help("configMap or secret")
        //         .index(1)
        //         .required(false)
        //         .requires("NAME"),
        // )
        .arg(
            Arg::with_name("NAME")
                .help("the name of the secret or configmap")
                // .index(2)
                .index(1)
        )
        .get_matches();

    UserArgs::new(
        matches.value_of("KUBECONFIG").map(|arg| arg.to_string()),
        matches.value_of("NAMESPACE").map(|arg| arg.to_string()),
        matches.value_of("OUTPUT").map_or(Output::Plain, |arg| {
            Output::from_str(arg).unwrap_or(Output::Plain)
        }),
        // matches.value_of("KIND").map_or(Kind::None, |arg| Kind::from_str(arg).unwrap_or(Kind::None)),
        matches.value_of("NAME").map(|arg| arg.to_string()),
    )
}

pub struct UserArgs {
    pub kubeconfig: Option<String>,
    pub namespace: Option<String>,
    pub output: Output,
    // pub kind: Kind,
    pub name: Option<String>,
}

impl UserArgs {
    pub fn new(kubeconfig: Option<String>, namespace: Option<String>, output: Output, /* kind: Kind, */ name: Option<String>) -> Self {
        UserArgs {
            kubeconfig,
            namespace,
            output,
            // kind,
            name,
        }
    }
}

pub enum Output {
    Yaml,
    Json,
    Plain,
}

pub enum Kind {
    None,
    ConfigMap,
    Secret,
}

impl FromStr for Output {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.trim().to_lowercase().as_str() {
            "yaml" => Ok(Output::Yaml),
            "json" => Ok(Output::Json),
            "plain" => Ok(Output::Plain),
            _ => Err("Invalid output format".to_string()),
        };
    }
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.trim().to_lowercase().as_str() {
            "" => Ok(Kind::None),
            "configmap" => Ok(Kind::ConfigMap),
            "cm" => Ok(Kind::ConfigMap),
            "secret" => Ok(Kind::Secret),
            _ => Err("Invalid kind".to_string()),
        };
    }
}
