use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dateparser::parse;
use lazy_static::lazy_static;

lazy_static! {
    static ref ACCEPTED: Vec<&'static str> = vec![
        "1511648546",
        "1620021848429",
        "1620024872717915000",
        "2021-05-01T01:17:02.604456Z",
        "2017-11-25T22:34:50Z",
        "Wed, 02 Jun 2021 06:31:39 GMT",
        "2019-11-29 08:08:05-08",
        "2021-05-02 23:31:36.0741-07",
        "2021-05-02 23:31:39.12689-07",
        "2019-11-29 08:15:47.624504-08",
        "2021-04-30 21:14:10",
        "2021-04-30 21:14:10.052282",
        "2017-11-25 13:31:15 PST",
        "2017-11-25 13:31 PST",
        "2021-02-21",
        "2021-02-21 PST",
        "01:06:06",
        "4:00pm",
        "6:00 AM",
        "01:06:06 PST",
        "4:00pm PST",
        "6:00 AM PST",
        "May 02, 2021 15:51:31 UTC",
        "May 02, 2021 15:51 UTC",
    ];
}

fn bench_parse_all(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("parse_all", "accepted_formats"),
        &ACCEPTED,
        |b, all| {
            b.iter(|| {
                for date_str in all.iter() {
                    let _ = parse(*date_str);
                }
            })
        },
    );
}

fn bench_parse_each(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_each");
    for date_str in ACCEPTED.iter() {
        group.bench_with_input(*date_str, *date_str, |b, input| b.iter(|| parse(input)));
    }
    group.finish();
}

criterion_group!(benches, bench_parse_all, bench_parse_each);
criterion_main!(benches);
