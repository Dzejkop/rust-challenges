use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
struct Args {
    dir: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let progress_bar = Arc::new(ProgressBar::new(0));
    progress_bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} [{elapsed_precise}] Files: {pos} {wide_msg}")?,
    );

    let file_contents = Arc::new(Mutex::new(Vec::new()));
    let is_done = Arc::new(AtomicBool::new(false));

    // Visit directories and collect file paths
    let visit_task: JoinHandle<eyre::Result<()>> = {
        let file_contents = file_contents.clone();
        let is_done = Arc::clone(&is_done);

        tokio::spawn(async move {
            visit_dirs(&args.dir, file_contents)?;

            is_done.store(true, std::sync::atomic::Ordering::Relaxed);

            eyre::Result::Ok(())
        })
    };

    // Update the progress bar
    let update_progress_bar_task = {
        let progress_bar = progress_bar.clone();
        let file_contents = file_contents.clone();
        let is_done = Arc::clone(&is_done);

        tokio::spawn(async move {
            while !is_done.load(Ordering::Relaxed) {
                sleep(Duration::from_secs_f32(0.1)).await;
                progress_bar.set_position(file_contents.lock().unwrap().len() as u64);
            }
        })
    };

    let (visit, update) = tokio::join!(visit_task, update_progress_bar_task);
    visit??; // join error & eyre error
    update?; // only join error

    let total_files = file_contents.lock().unwrap().len();
    progress_bar.set_length(total_files as u64);
    progress_bar.finish_with_message("Completed");

    Ok(())
}

fn visit_dirs(dir: &Path, file_contents: Arc<Mutex<Vec<Vec<u8>>>>) -> eyre::Result<()> {
    let rd = fs::read_dir(dir)?;

    for entry in rd {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let content = std::fs::read(entry.path())?;
            std::thread::sleep(Duration::from_secs_f32(0.001));

            file_contents.lock().unwrap().push(content);
        } else if entry.file_type()?.is_dir() {
            visit_dirs(&entry.path(), file_contents.clone())?;
        } else {
            continue;
        }
    }

    Ok(())
}
