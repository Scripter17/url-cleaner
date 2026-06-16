use crate::prelude::*;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(long)]
    pub num: u32,
    #[clap(long)]
    pub new: bool,
    #[clap(long)]
    pub old: bool,
    #[clap(long)]
    pub raw: bool,
}

impl Args {
    pub fn r#do(self) {
        let urls = [
            "httPs://a:b@1:3/a",
            "https://example.com",
            "https://example.com?#",
            "https://example.com?a#a",
            "https://example.com/asdakjdfsd/f/sdf/sdf/sdfsdafsdfsdf/sdaf/sdafsdgffsdg/fsdg/fsdg/fsdg/fsdgfsdgfdgfdsgfsd/gfsd/gfd/gfsdg",
            "test:abc",
        ];

        for url in urls {
            println!("{url}");

            if self.new {
                let a = std::time::Instant::now();
                for _ in 0..self.num {MyUrl::new(url).unwrap();}
                println!("  New: {:?}", a.elapsed());
            }

            if self.old {
                let a = std::time::Instant::now();
                for _ in 0..self.num {BetterUrl::parse(url).unwrap();}
                println!("  Old: {:?}", a.elapsed());
            }

            if self.raw {
                let a = std::time::Instant::now();
                for _ in 0..self.num {url::Url::parse(url).unwrap();}
                println!("  Raw: {:?}", a.elapsed());
            }

            println!();
        }
    }
}
