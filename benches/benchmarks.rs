use criterion::{black_box, criterion_group, criterion_main, Criterion};
use encoding_rs::WINDOWS_874;
use rand::seq::SliceRandom;

const ALL_THAI_CHARS: &str =
    "กขฃคฅฆงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรฤลฦวศษสหฬอฮฯะัาำิีึืฺุู฿เแโใไๅๆ็่้๊๋์ํ๎๏๐๑๒๓๔๕๖๗๘๙๚๛";

fn prepare_input(len: usize) -> String {
    let mut characters: Vec<char> = (32..127).map(|i| std::char::from_u32(i).unwrap()).collect();
    characters.extend(ALL_THAI_CHARS.chars());
    let mut out = Vec::with_capacity(len);
    while out.len() < len {
        out.extend(characters.iter().copied());
    }
    out.truncate(len);
    let mut rng = &mut rand::thread_rng();
    out.as_mut_slice().shuffle(&mut rng);
    out.into_iter().collect()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let message = prepare_input(2_000_000);
    let encoded = tis620::encode(&message).unwrap();
    let encoded2 = WINDOWS_874.encode(black_box(&message)).0;
    assert_eq!(encoded.as_slice(), encoded2.as_ref());
    let decoded = tis620::decode(&encoded).unwrap();
    let decoded2 = WINDOWS_874.decode(&encoded).0;
    assert_eq!(decoded, decoded2);
    drop(encoded2);
    drop(decoded);
    drop(decoded2);
    c.bench_function("encode", |b| {
        b.iter(|| tis620::encode(black_box(&message)).unwrap())
    });
    c.bench_function("encoding.rs encode", |b| {
        b.iter(|| WINDOWS_874.encode(black_box(&message)))
    });
    c.bench_function("decode", |b| {
        b.iter(|| tis620::decode(black_box(&encoded)).unwrap())
    });
    c.bench_function("encoding.rs decode", |b| {
        b.iter(|| WINDOWS_874.decode(black_box(&encoded)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
