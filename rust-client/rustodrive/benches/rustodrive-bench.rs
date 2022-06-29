use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(feature = "mock-socket")]
fn bench_can_proxy(c: &mut Criterion) {

    use rustodrive::{
        canproxy::CANProxy,
        commands::{ODriveCommand, Write},
        messages::ODriveCANFrame,
    };

    let mut can_proxy = CANProxy::new("fakecan");
    can_proxy.register_rw("thread 1", |can_read_write| {
        while can_read_write.check_alive() {
            can_read_write.request(ODriveCANFrame {
                axis: 1,
                cmd: ODriveCommand::Write(Write::SetInputVelocity),
                data: [0; 8],
            });
        }
        println!("blah");
    });
    c.bench_function("send request", |b| {
        b.iter(|| {
            can_proxy.process_messages();
        });
    });
    // FIXME `cargo bench` does not exit even after calling stop
    can_proxy.stop();

}

#[cfg(not(feature = "mock-socket"))]
fn bench_can_proxy() {}


criterion_group!(benches, bench_can_proxy);
criterion_main!(benches);
