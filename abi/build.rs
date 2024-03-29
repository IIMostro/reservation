use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        // package name + 类型的名称
        .type_attribute("reservation.ReservationStatus", "#[derive(sqlx::Type)]")
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    Command::new("cargo")
        .args(&["fmt"])
        .output()
        .expect("failed to execute process");
    println!("cargo:rerun-if-changed=protos/reservation.proto");
}
