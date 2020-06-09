use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_index_cmd_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastatools")?;
    cmd.arg("index").arg("resources/test.fasta");
    cmd.assert().stderr(predicate::str::contains("All done."));
    Ok(())
}

// #[test]
// fn whole_workflow() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("kit")?;
//     cmd.arg("add").arg("X").arg("30").arg("2020-1-1").arg("-n");
//     // cmd.arg("add").arg("X").arg("30").arg("now");
//     cmd.assert().stderr(predicate::str::contains("Added \"X\""));

//     let mut cmd = Command::cargo_bin("kit")?;
//     cmd.arg("modify").arg("X").arg("interval").arg("10");
//     cmd.assert()
//         .stderr(predicate::str::contains("Modified \"X\""));

//     let mut cmd = Command::cargo_bin("kit")?;
//     cmd.arg("just-talked-to").arg("X");
//     cmd.assert()
//         .stderr(predicate::str::contains("Modified \"X\""));

//     let mut cmd = Command::cargo_bin("kit")?;
//     cmd.arg("modify").arg("X").arg("last").arg("2015-10-10");
//     cmd.assert()
//         .stderr(predicate::str::contains("Modified \"X\""));

//     let mut cmd = Command::cargo_bin("kit")?;
//     cmd.arg("remove").arg("X");
//     cmd.assert()
//         .stderr(predicate::str::contains("Removed \"X\""));
//     Ok(())
// }
