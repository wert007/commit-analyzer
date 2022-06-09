use std::{collections::HashMap, io::Write, ops::AddAssign};

use clap::Parser;

fn main() -> sysexits::ExitCode {
    let mut args = commit_analyzer::Args::parse();

    let commits = match args.input_method.read() {
        Ok(string) => string,
        Err(_) => match args.input_method {
            commit_analyzer::InputMethod::GitHistory => {
                eprintln!("Reading from the Git history was not possible.");
                return sysexits::ExitCode::Unavailable;
            }
            commit_analyzer::InputMethod::LogFile { log_file: input } => {
                eprintln!("The input file '{}' could not be read.", input.display());
                return sysexits::ExitCode::NoInput;
            }
            commit_analyzer::InputMethod::Stdin => {
                eprintln!("Reading from `stdin` failed.");
                return sysexits::ExitCode::IoErr;
            }
        },
    };
    let mut commits = commits.as_str();
    let mut parsed_commits = vec![];
    while !commits.is_empty() {
        let result = commit_analyzer::Commit::parse(commits);
        match result {
            Ok((commit, remainder)) => {
                commits = remainder;
                parsed_commits.push(commit);
            }
            Err(err) => {
                dbg!(err);
                break;
            }
        }
    }
    let mut last_time = None;
    let mut duration = chrono::Duration::zero();
    let output = args.output.take();
    let is_verbose = args.verbose;
    let max_time_difference = args.duration as i64;
    let filter = commit_analyzer::Filter::new(args);
    let mut commit_count = 0;
    let mut commits_per_day = HashMap::new();
    let mut loc_per_day = HashMap::new();
    for commit in parsed_commits.into_iter().rev() {
        if filter.matches(&commit) {
            commit_count += 1;
            commits_per_day
                .entry(commit.date().date())
                .or_insert(0)
                .add_assign(1);
            loc_per_day
                .entry(commit.date().date())
                .or_insert(0)
                .add_assign(commit.loc(&filter));
            if let Some(last_time) = last_time {
                let diff: chrono::Duration = *commit.date() - last_time;
                if diff.num_hours() <= max_time_difference {
                    duration = duration + diff;
                }
            }
            last_time = Some(*commit.date());
            if is_verbose {
                println!("{:#?}", commit);
            }
        }
    }

    println!("Estimated time was {}h", duration.num_hours());
    println!("Found {} commits overall", commit_count);

    if let Some(path) = output {
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(_) => return sysexits::ExitCode::CantCreat,
        };
        let mut sorted_per_day_data = vec![];
        for key in commits_per_day.keys() {
            let commit_count = commits_per_day[key];
            let loc = loc_per_day[key];
            sorted_per_day_data.push((*key, commit_count, loc));
        }
        sorted_per_day_data.sort_by_cached_key(|(k, _, _)| *k);
        match writeln!(file, "Date, Commits, Loc") {
            Ok(something) => something,
            Err(_) => return sysexits::ExitCode::IoErr,
        };
        for (date, commits, loc) in sorted_per_day_data {
            match writeln!(file, "{}, {}, {}", date, commits, loc) {
                Ok(something) => something,
                Err(_) => return sysexits::ExitCode::IoErr,
            };
        }
    }

    sysexits::ExitCode::Ok
}
