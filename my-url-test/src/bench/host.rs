use crate::prelude::*;

#[derive(Debug, Parser)]
pub struct Args {
    pub hosts: Vec<String>,
    #[arg(long, default_value_t = 10_000)]
    pub num: usize,
    #[arg(long)]
    pub new: bool,
    #[arg(long)]
    pub raw: bool,
}

impl Args {
    pub fn r#do(self) {
        for host in &self.hosts {
            println!("{host}");

            assert_eq!(Host::new(&**host).unwrap(), url::Host::parse(host).unwrap().to_string());

            if self.new {
                let a = Instant::now();
                for _ in 0..self.num {
                    Host::new(&**host).unwrap();
                }
                println!("  New: {:?}", a.elapsed());
            }

            if self.raw {
                let a = Instant::now();
                for _ in 0..self.num {
                    url::Host::parse(host).unwrap();
                }
                println!("  Raw: {:?}", a.elapsed());
            }

            println!();
        }
    }
}
