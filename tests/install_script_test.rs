// Copyright (C) 2025 princepal9120
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, version 3 of the License.

#![cfg(unix)]

use sha2::{Digest, Sha256};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn unix_installer_verifies_checksum_file_with_release_asset_name() {
    let dir = tempdir().unwrap();
    let home = dir.path().join("home");
    let fake_bin = dir.path().join("bin");
    fs::create_dir_all(&home).unwrap();
    fs::create_dir_all(&fake_bin).unwrap();

    let os = match std::env::consts::OS {
        "macos" => "macos",
        "linux" => "linux",
        other => panic!("unsupported unix test OS: {other}"),
    };
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        other => panic!("unsupported unix test architecture: {other}"),
    };
    let asset_name = format!("devrunner-{os}-{arch}");
    let binary = b"fake devrunner binary";
    let checksum = format!("{:x}  {asset_name}\n", Sha256::digest(binary));

    fs::write(dir.path().join(&asset_name), binary).unwrap();
    fs::write(dir.path().join(format!("{asset_name}.sha256")), checksum).unwrap();

    let fake_curl = fake_bin.join("curl");
    fs::write(
        &fake_curl,
        format!(
            r#"#!/usr/bin/env bash
set -euo pipefail
out=""
url=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    -o) out="$2"; shift 2 ;;
    -*) shift ;;
    *) url="$1"; shift ;;
  esac
done
if [[ "$url" == *"/releases/latest" ]]; then
  printf '{{"tag_name":"v-test"}}'
elif [[ "$url" == *"{asset_name}.sha256" ]]; then
  cp "{root}/{asset_name}.sha256" "$out"
elif [[ "$url" == *"{asset_name}" ]]; then
  cp "{root}/{asset_name}" "$out"
else
  echo "unexpected url: $url" >&2
  exit 1
fi
"#,
            root = dir.path().display(),
        ),
    )
    .unwrap();
    let mut perms = fs::metadata(&fake_curl).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&fake_curl, perms).unwrap();

    let fake_path = format!(
        "{}:{}",
        fake_bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );

    let output = Command::new("bash")
        .arg("install.sh")
        .env("HOME", &home)
        .env("PATH", fake_path)
        .env("SHELL", "/bin/bash")
        .env("OSTYPE", "darwin")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "installer failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(fs::read(home.join(".local/bin/devrunner")).unwrap(), binary);
    assert!(home.join(".local/bin/dr").exists());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Checksum verified"));
}
