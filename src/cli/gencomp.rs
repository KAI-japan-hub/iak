//! シェル補完ファイルの生成処理

use crate::args::Args;
use clap::{Command, CommandFactory};
use clap_complete::{generate as gen_completion, Shell};
use std::path::Path;

fn generate_impl(shell: Shell, cmd: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    if let Ok(mut dest) = std::fs::File::create(destfile) {
        gen_completion(shell, cmd, appname, &mut dest);
    }
}

/// bash/zsh/fish/elvish/powershell 向けの補完ファイルを一括生成する
pub fn generate_all(outdir: &Path) {
    use Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
    let appname = "iak";
    let mut cmd = Args::command();
    cmd.set_bin_name(appname);
    generate_impl(Bash, &mut cmd, appname, outdir, format!("bash/{appname}"));
    generate_impl(Elvish, &mut cmd, appname, outdir, format!("elvish/{appname}"));
    generate_impl(Fish, &mut cmd, appname, outdir, format!("fish/{appname}"));
    generate_impl(PowerShell, &mut cmd, appname, outdir, format!("powershell/{appname}"));
    generate_impl(Zsh, &mut cmd, appname, outdir, format!("zsh/_{appname}"));
}
