pub struct Cleaning<'a> {
    cleaner: &'a Cleaner,
    context: CleaningContext,
    jobs: Box<dyn Iterator<Item = JobConfigMaker>>
}

impl<'a> Cleaning<'a> {
    pub fn iter(&'a mut self) -> impl Iterator<Item = Result<Job<'a>, MakeJobConfigError>> {
        (&mut self.jobs).into_iter()
            .map(|job_config_maker| )
    }
}

pub struct Cleaner {
    config: CleanerConfig,
    cache: Cache
}

pub struct CleanerConfig {
    docs: CleanerDocs,
    params: Params,
    commons: Commons,
    rules: Rules
}

pub struct CleaningContext {
    vars: HashMap<String, String>
}

pub enum JobConfigMaker {
    Made(JobConfig),
    JsonString(String),
    JsonValue(Value)
}

pub struct JobConfig {
    url: BetterUrl,
    context: JobContext
}

impl JobConfigMaker {
    pub fn make(self) -> Result<JobConfig, MakeJobConfigError> {
        todo!()
    }
}

pub enum MakeJobConfigError {
    JsonError(serde_json::Error)
}

pub enum LazyJobConfig {
    Unmade(JobConfigMaker),
    Made(JobConfig)
}



pub struct Job<'a> {
    cleaner: &'a Cleaner,
    url: BetterUrl,
    context: JobContext,
    stack: RwLock<Vec<CommonCallArgs<'a>>>
}

impl Job<'_> {
    pub fn r#do(mut self) -> Result<BetterUrl, DoJobError> {
        self.cleaner.config.rules.apply(&mut self)?;

        Ok(self.url)
    }
}
