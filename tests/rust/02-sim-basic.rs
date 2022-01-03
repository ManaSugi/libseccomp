// SPDX-License-Identifier: LGPL-2.1-only
//
// Copyright 2024 Sony Group Corporation
//
// Seccomp Library test program
//

use anyhow::Result;
use libseccomp::*;
use utils::*;

fn main() -> Result<()> {
    let opts = util_getopt();
    let mut ctx = ScmpFilterContext::new(ScmpAction::KillThread)?;

    ctx.add_rule_exact(ScmpAction::Allow, ScmpSyscall::from_name("read")?)?;
    ctx.add_rule_exact(ScmpAction::Allow, ScmpSyscall::from_name("write")?)?;
    ctx.add_rule_exact(ScmpAction::Allow, ScmpSyscall::from_name("close")?)?;
    ctx.add_rule_exact(ScmpAction::Allow, ScmpSyscall::from_name("rt_sigreturn")?)?;

    util_filter_output(&opts, &ctx)
}