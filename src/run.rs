use rocket::tokio;
use tokio::io::AsyncWriteExt;

use crate::Config;
use crate::Error;
use crate::Output;

pub async fn run(
    code: impl AsRef<str>,
    semaphore: &tokio::sync::Semaphore,
    config: &Config,
) -> Result<Output, Error> {
    let code = code.as_ref();

    let _semaphore = loop {
        match semaphore.try_acquire() {
            Ok(ok) => break ok,
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_millis(config.semaphore_wait as u64)).await
            }
        }
    };

    if code.contains("::std") {
        return Err(Error::Std);
    } else if code.contains("::core") {
        return Err(Error::Core);
    } else if code.contains("extern \"C\"") {
        return Err(Error::ExternC);
    } else if code.contains("unsafe") {
        return Err(Error::Unsafe);
    }

    let Ok(scratch) = tempfile::Builder::new().prefix("playground").tempdir() else {
        return Err(Error::TempDir);
    };
    let Ok(input_file) = create_project_template(&scratch).await else {
        return Err(Error::InputFileCreate);
    };
    let Ok(mut input_file) = tokio::fs::OpenOptions::new()
        .append(true)
        .open(input_file)
        .await
    else {
        return Err(Error::InputFileOpen);
    };
    if input_file.write_all(code.as_bytes()).await.is_err() {
        return Err(Error::InputFileWrite);
    }
    drop(input_file);

    let Ok(compile) = tokio::process::Command::new("cargo")
        .args(["build"])
        .kill_on_drop(true)
        .current_dir(&scratch)
        .output()
        .await
    else {
        return Err(Error::Build);
    };
    if !compile.status.success() {
        return Err(Error::Compiler(String::from_utf8(compile.stderr).unwrap()));
    }

    let run = tokio::process::Command::new("cargo")
        .arg("run")
        .kill_on_drop(true)
        .current_dir(&scratch)
        .output();

    // TODO: Panic does not trigger Err
    match tokio::time::timeout(
        std::time::Duration::from_millis(config.kill_timeout as u64),
        run,
    )
    .await
    {
        Ok(Ok(result)) => Ok(Output {
            stdout: String::from_utf8(result.stdout).unwrap(),
            stderr: String::from_utf8(result.stderr).unwrap(),
        }),
        Ok(Err(e)) => Err(Error::Execution(e.to_string())),
        Err(_) => Err(Error::Timeout),
    }
}

async fn create_project_template(
    parent: impl AsRef<std::path::Path>,
) -> std::io::Result<std::path::PathBuf> {
    let template = concat!(env!("CARGO_MANIFEST_DIR"), "/project-template/.");
    tokio::process::Command::new("cp")
        .arg("-r")
        .arg(template)
        .arg(parent.as_ref())
        .spawn()?
        .wait()
        .await?;
    Ok(parent.as_ref().join("src/main.rs"))
}
