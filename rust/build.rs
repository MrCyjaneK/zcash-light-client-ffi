extern crate cbindgen;

use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("os_log_.*")
        .allowlist_function("os_release")
        .allowlist_function("os_signpost_.*")
        .generate()
        .expect("should be able to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("should be able to write bindings");

    cc::Build::new().file("wrapper.c").compile("wrapper");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if let Ok(b) = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .rename_item("Account", "ZCASH_Account")
        .rename_item("Uuid", "ZCASH_Uuid")
        .rename_item("Accounts", "ZCASH_Accounts")
        .rename_item("BinaryKey", "ZCASH_BinaryKey")
        .rename_item("EncodedKey", "ZCASH_EncodedKey")
        .rename_item("EncodedKeys", "ZCASH_EncodedKeys")
        .rename_item("SubtreeRoot", "ZCASH_SubtreeRoot")
        .rename_item("SubtreeRoots", "ZCASH_SubtreeRoots")
        .rename_item("Balance", "ZCASH_Balance")
        .rename_item("AccountBalance", "ZCASH_AccountBalance")
        .rename_item("ScanProgress", "ZCASH_ScanProgress")
        .rename_item("WalletSummary", "ZCASH_WalletSummary")
        .rename_item("ScanRange", "ZCASH_ScanRange")
        .rename_item("ScanRanges", "ZCASH_ScanRanges")
        .rename_item("ScanSummary", "ZCASH_ScanSummary")
        .rename_item("BlockMeta", "ZCASH_BlockMeta")
        .rename_item("BlocksMeta", "ZCASH_BlocksMeta")
        .rename_item("BoxedSlice", "ZCASH_BoxedSlice")
        .rename_item("TxIds", "ZCASH_TxIds")
        .rename_item("MaxSpendMode", "ZCASH_MaxSpendMode")
        .rename_item("TransactionStatus", "ZCASH_TransactionStatus")
        .rename_item("TransactionDataRequest", "ZCASH_TransactionDataRequest")
        .rename_item("TransactionDataRequests", "ZCASH_TransactionDataRequests")
        .rename_item("Address", "ZCASH_Address")
        .rename_item("AccountMetadataKey", "ZCASH_AccountMetadataKey")
        .rename_item("SymmetricKeys", "ZCASH_SymmetricKeys")
        .rename_item("HttpRequestHeader", "ZCASH_HttpRequestHeader")
        .rename_item("HttpResponseBytes", "ZCASH_HttpResponseBytes")
        .rename_item("HttpResponseHeader", "ZCASH_HttpResponseHeader")
        .rename_item("SingleUseTaddr", "ZCASH_SingleUseTaddr")
        .rename_item("AddressCheckResult", "ZCASH_AddressCheckResult")
        .rename_item("TransactionStatusFilter", "ZCASH_TransactionStatusFilter")
        .rename_item("OutputStatusFilter", "ZCASH_OutputStatusFilter")
        .rename_item("TorDormantMode", "ZCASH_TorDormantMode")
        .rename_item("LwdConn", "ZCASH_LwdConn")
        .rename_item("TorRuntime", "ZCASH_TorRuntime")
        .rename_item("ConfirmationsPolicy", "ZCASH_ConfirmationsPolicy")
        .rename_item(
            "TransactionsInvolvingAddress_Body",
            "ZCASH_TransactionsInvolvingAddress_Body",
        )
        .rename_item("Decimal", "ZCASH_Decimal")
        .generate()
    {
        b.write_to_file("target/Headers/zcashlc.h");
    }
}
