use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_index_cmd_success_uniprot() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastatools")?;
    cmd.arg("index").arg("resources/test.fasta");
    cmd.assert().stderr(predicate::str::contains("All done."));
    Ok(())
}

#[test]
fn test_index_cmd_success_id_only() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastatools")?;
    cmd.arg("index").arg("resources/test_short_desc.fasta");
    cmd.assert().stderr(predicate::str::contains("All done."));
    Ok(())
}

#[test]
fn test_accessions_cmd_success_id_only() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastatools")?;
    cmd.arg("accessions").arg("resources/test.fasta");
    cmd.assert().stderr(predicate::str::contains("All done."));
    Ok(())
}

#[test]
fn test_accessions_cmd_success_uniprot() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastatools")?;
    cmd.arg("accessions").arg("resources/test_short_desc.fasta");
    cmd.assert().stderr(predicate::str::contains("All done."));
    Ok(())
}

#[test]
fn test_get_entry_cmd_success_uniprot() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastatools")?;
    cmd.arg("get-entry")
        .arg("resources/test.fasta")
        .arg("resources/test_index.json")
        .arg("H0VS30");
    let descr = "tr|H0VS30|H0VS30_CAVPO Receptor protein serine/threonine kinase OS=Cavia porcellus OX=10141 GN=TGFBR1 PE=3 SV=2";
    let seq = "MEAAAAAPRHQLLLLMLVAAAATLLPGAKALQCFCQLCAKDNYTCVTDGLCFVSITETTDRIIHNTMCIAEIDLIPRDRPFVCAPSSKTGAVTTTHCCNQDHCNKIELPTTEKQS\
    SGLGPVELAAVIAGPVCFVCISLMLMVYICHNRTVIHHRVPNEEDPSLDRPFISEGTTLKDLIYDMTTSGSGSGLPLLVQRTIARTIVLQESIGKGRFGEVWRGKWRGEEVAVKIFSSREERSWFR\
    EAEIYQTVMLRHENILGFIAADNKDNGTWTQLWLVSDYHEHGSLFDYLNRYTVTVEGMIKLALSTASGLAHLHMEIVGTQGKPAIAHRDLKSKNILVKKNGTCCIADLGLAVRHDSATDTIDIAPN\
    HRVGTKRYMAPEVLDDSINMKHFESFKRADIYAMGLVFWEIARRCSIGGIHEDYQLPYYDLVPSDPSVEEMRKVVCEQKLRPNIPNRWQSCEALRVMAKIMRECWYANGAARLTALRIKKTLSQLS\
    QQEGIKMSFVVCH";
    cmd.assert().stdout(predicate::str::contains(descr));
    cmd.assert().stdout(predicate::str::contains(seq));
    Ok(())
}

#[test]
fn test_subset_cmd_success_uniprot() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastatools")?;
    cmd.arg("subset")
        .arg("resources/test.fasta")
        .arg("resources/test_index.json")
        .arg("resources/subset_ids.txt");
    let descr_a = "tr|H0VS30|H0VS30_CAVPO Receptor protein serine/threonine kinase OS=Cavia porcellus OX=10141 GN=TGFBR1 PE=3 SV=2";
    let seq_a = "MEAAAAAPRHQLLLLMLVAAAATLLPGAKALQCFCQLCAKDNYTCVTDGLCFVSITETTD\
    RIIHNTMCIAEIDLIPRDRPFVCAPSSKTGAVTTTHCCNQDHCNKIELPTTEKQSSGLGP\
    VELAAVIAGPVCFVCISLMLMVYICHNRTVIHHRVPNEEDPSLDRPFISEGTTLKDLIYD\
    MTTSGSGSGLPLLVQRTIARTIVLQESIGKGRFGEVWRGKWRGEEVAVKIFSSREERSWF\
    REAEIYQTVMLRHENILGFIAADNKDNGTWTQLWLVSDYHEHGSLFDYLNRYTVTVEGMI\
    KLALSTASGLAHLHMEIVGTQGKPAIAHRDLKSKNILVKKNGTCCIADLGLAVRHDSATD\
    TIDIAPNHRVGTKRYMAPEVLDDSINMKHFESFKRADIYAMGLVFWEIARRCSIGGIHED\
    YQLPYYDLVPSDPSVEEMRKVVCEQKLRPNIPNRWQSCEALRVMAKIMRECWYANGAARL\
    TALRIKKTLSQLSQQEGIKMSFVVCH";
    let descr_b = "tr|Q8I5U1|Q8I5U1_PLAF7 GPI mannosyltransferase 1 OS=Plasmodium falciparum (isolate 3D7) OX=36329 GN=PF3D7_1210900 PE=3 SV=2";
    let seq_b = "MGHTHKEYKNNEKSAIFFEWLIFFVGIIIRIIIYYYGRWQDKNFNVKFTDVDYYVFSDAA\
    KYVLMNKSPYERYTYRYTPLLAYIMIPNFFVHFSFGKILFSFIDILVTILINQIIKIKYT\
    NCKNYIFYTCLWFLNPLVIIISLRGNADVIPCFLIIVTIFCIYKKHIFLSSIFYGLAVNF\
    KIYTIIYALPFMLYLNKNYLLGENIFQLNEKKKKKKNDFLLNTFFYIFRIISNFFVELFK\
    LNYEQFLFAICSSSVFLILNCVFYIIYGYEFLYESFIYHIIRRDHRHNFSLFFYLMYLSI\
    EKNSKIIPLITFVPQIILVALFGFKYARTNLELSMFLQTISFIALNKVCTSQYFIWCIPF\
    LPIILCAITLSKRNMFLIISSILFFIVAKLHWLWWAYYLEFRGYNTFLQLFYSSVLFVIS\
    EISICWVFMYINFKEQKTKIT";
    cmd.assert().stdout(predicate::str::contains(descr_a));
    cmd.assert().stdout(predicate::str::contains(seq_a));
    cmd.assert().stdout(predicate::str::contains(descr_b));
    cmd.assert().stdout(predicate::str::contains(seq_b));
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
