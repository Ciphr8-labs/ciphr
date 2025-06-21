use criterion::{black_box, criterion_group, criterion_main, Criterion};
use config::{
    builder::AppConfigBuilder,
    loader::FileConfigurationProvider,
    traits::ConfigurationProvider,
    types::{LogFormat, LogLevel},
};
use std::io::Write;
use tempfile::NamedTempFile;

fn create_temp_config_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", content).unwrap();
    file
}

fn bench_config_loading(c: &mut Criterion) {
    let content = r#"
        environment = "production"
        log_level = "warn"
        log_format = "json"
        [feature_flags]
        new_feature = true
        beta_access = false
    "#;
    let file = create_temp_config_file(content);
    let provider = FileConfigurationProvider::new(file.path());

    c.bench_function("config_load_from_file", |b| {
        b.iter(|| provider.load().unwrap())
    });
}

fn bench_config_builder(c: &mut Criterion) {
    c.bench_function("config_build_with_builder", |b| {
        b.iter(|| {
            AppConfigBuilder::new()
                .environment(black_box("production"))
                .log_level(black_box(LogLevel::Warn))
                .log_format(black_box(LogFormat::Json))
                .feature_flag(black_box("new_feature"), black_box(true))
                .build()
        })
    });
}

criterion_group!(benches, bench_config_loading, bench_config_builder);
criterion_main!(benches); 