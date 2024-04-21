use libjsandbox::script::AnyError;

#[tokio::test]
async fn call_06() -> Result<(), AnyError> {
    let num_cpu = num_cpus::get();
    dbg!(num_cpu);
    assert_eq!(num_cpu, 16);

    let num_cpu_physical = num_cpus::get_physical();
    dbg!(num_cpu_physical);
    assert_eq!(num_cpu_physical, 8);

    Ok(())
}
